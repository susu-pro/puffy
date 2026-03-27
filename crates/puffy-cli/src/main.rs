use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use puffy_core::{
    asset_job::AssetJobRequest,
    asset_pipeline,
    default_api_base, engine_overview,
    runtime::{rehearsal_runtime_lock, runtime_rules_summary},
};

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("serve") => {
            let addr = env::var("PUFFY_API_ADDR").unwrap_or_else(|_| "127.0.0.1:41480".to_string());
            if let Err(error) = serve(&addr) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        Some("doctor") => {
            print_doctor();
        }
        Some("extract") => {
            if let Err(error) = run_extract(args) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        Some("runtime") => {
            println!("{:#?}", rehearsal_runtime_lock());
        }
        Some("--help") | None => {
            print_usage();
        }
        Some(other) => {
            eprintln!("Unknown command: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    println!(
        "Puffy public engine rehearsal\n\nCommands:\n  doctor\n  serve\n  extract <url> [--save-dir <dir>] [--profile auto|1080p|720p|audio-only]\n  runtime"
    );
}

fn print_doctor() {
    let lock = rehearsal_runtime_lock();
    println!("{}", engine_overview());
    println!("{}", runtime_rules_summary());
    println!("runtime entries: {}", lock.tools.len());
    println!("api base: {}", default_api_base());
}

fn run_extract(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let Some(url) = args.next() else {
        return Err("extract requires a URL".to_string());
    };

    let mut save_dir = None;
    let mut profile = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--save-dir" => {
                save_dir = args.next();
            }
            "--profile" => {
                profile = args.next();
            }
            other => {
                return Err(format!("Unknown flag: {other}"));
            }
        }
    }

    let request = AssetJobRequest {
        job_id: None,
        source_url: url,
        save_dir,
        download_profile: profile,
        force_rerun: false,
    };
    let plan = asset_pipeline::plan_asset_job(&request)?;
    println!(
        "planned job -> url={}, save_dir={}, profile={}, stages={}",
        plan.download_plan.source_url,
        plan.save_dir,
        plan.download_plan.profile.label(),
        plan.stages.len()
    );
    Ok(())
}

fn serve(addr: &str) -> Result<(), String> {
    let listener = TcpListener::bind(addr).map_err(|error| format!("bind {addr} failed: {error}"))?;
    println!("Puffy public engine listening on http://{addr}");
    let jobs = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    for incoming in listener.incoming() {
        let stream = incoming.map_err(|error| format!("accept failed: {error}"))?;
        let jobs = jobs.clone();
        thread::spawn(move || {
            if let Err(error) = handle_connection(stream, jobs) {
                eprintln!("{error}");
            }
        });
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    jobs: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), String> {
    let (method, path, body) = read_request(&mut stream)?;
    let (status, response_body) = route_request(&method, &path, &body, &jobs)?;
    write_response(&mut stream, status, &response_body)
}

fn read_request(stream: &mut TcpStream) -> Result<(String, String, String), String> {
    let mut reader = BufReader::new(stream.try_clone().map_err(|error| format!("clone stream failed: {error}"))?);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|error| format!("read request line failed: {error}"))?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("malformed HTTP request".to_string());
    }
    let method = parts[0].to_string();
    let path = parts[1].to_string();

    let mut content_length = 0usize;
    loop {
        let mut header_line = String::new();
        let bytes = reader
            .read_line(&mut header_line)
            .map_err(|error| format!("read header failed: {error}"))?;
        if bytes == 0 || header_line == "\r\n" || header_line == "\n" {
            break;
        }
        let lower = header_line.to_ascii_lowercase();
        if let Some(value) = lower.strip_prefix("content-length:") {
            content_length = value.trim().parse::<usize>().unwrap_or(0);
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .map_err(|error| format!("read body failed: {error}"))?;
    }
    Ok((method, path, String::from_utf8_lossy(&body).to_string()))
}

fn route_request(
    method: &str,
    path: &str,
    body: &str,
    jobs: &Arc<Mutex<HashMap<String, String>>>,
) -> Result<(u16, String), String> {
    let response = match (method, path) {
        ("GET", "/health") | ("GET", "/api/health") => (
            200,
            json_object(&[
                ("status", json_string("ok")),
                ("name", json_string("Puffy Public Engine")),
                ("version", json_string(puffy_core::ENGINE_VERSION)),
            ]),
        ),
        ("GET", "/api/assets") => (
            200,
            json_object(&[
                ("status", json_string("success")),
                ("assets", "[]".to_string()),
            ]),
        ),
        ("GET", "/api/search") => (
            200,
            json_object(&[
                ("status", json_string("success")),
                ("hits", "[]".to_string()),
            ]),
        ),
        ("POST", "/api/extract") => handle_extract_request(body, jobs)?,
        _ if method == "GET" && path.starts_with("/api/jobs/") => {
            let job_id = path.trim_start_matches("/api/jobs/");
            let status = jobs
                .lock()
                .map_err(|error| format!("job store lock failed: {error}"))?
                .get(job_id)
                .cloned()
                .unwrap_or_else(|| "queued".to_string());
            (
                200,
                json_object(&[
                    ("status", json_string("success")),
                    ("job", json_object(&[
                        ("jobId", json_string(job_id)),
                        ("status", json_string(&status)),
                    ])),
                ]),
            )
        }
        _ => (
            404,
            json_object(&[
                ("status", json_string("error")),
                ("error", json_string("Not found")),
            ]),
        ),
    };

    Ok(response)
}

fn handle_extract_request(
    body: &str,
    jobs: &Arc<Mutex<HashMap<String, String>>>,
) -> Result<(u16, String), String> {
    let Some(url) = extract_json_string_field(body, "url") else {
        return Ok((
            400,
            json_object(&[
                ("status", json_string("error")),
                ("error", json_string("Missing url field.")),
            ]),
        ));
    };

    let request = AssetJobRequest {
        job_id: None,
        source_url: url.clone(),
        save_dir: None,
        download_profile: None,
        force_rerun: false,
    };

    let plan = match asset_pipeline::plan_asset_job(&request) {
        Ok(plan) => plan,
        Err(error) => {
            return Ok((
                400,
                json_object(&[
                    ("status", json_string("error")),
                    ("error", json_string(&error)),
                ]),
            ));
        }
    };

    let job_id = puffy_core::asset_job::generate_job_id();
    jobs.lock()
        .map_err(|error| format!("job store lock failed: {error}"))?
        .insert(job_id.clone(), "queued".to_string());

    let _ = plan;
    Ok((
        202,
        json_object(&[
            ("status", json_string("queued")),
            ("jobId", json_string(&job_id)),
            ("message", json_string("Asset job queued. Poll /api/jobs/{job_id} for progress.")),
            ("checkUrl", json_string(&format!("/api/jobs/{job_id}"))),
        ]),
    ))
}

fn extract_json_string_field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = body.find(&needle)?;
    let remainder = &body[start + needle.len()..];
    let colon = remainder.find(':')?;
    let after_colon = remainder[colon + 1..].trim_start();
    if !after_colon.starts_with('"') {
        return None;
    }

    let mut escaped = false;
    let mut output = String::new();
    for ch in after_colon[1..].chars() {
        if escaped {
            output.push(match ch {
                '"' => '"',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return Some(output),
            other => output.push(other),
        }
    }
    None
}

fn write_response(stream: &mut TcpStream, status: u16, body: &str) -> Result<(), String> {
    let status_text = match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "OK",
    };
    let response = format!(
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream
        .write_all(response.as_bytes())
        .map_err(|error| format!("write response failed: {error}"))?;
    Ok(())
}

fn json_object(fields: &[(&str, String)]) -> String {
    let body = fields
        .iter()
        .map(|(key, value)| format!("\"{}\":{}", escape_json(key), value))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{body}}}")
}

fn json_string(value: &str) -> String {
    format!("\"{}\"", escape_json(value))
}

fn escape_json(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            other => output.push(other),
        }
    }
    output
}
