# Puffy Public Engine

Paste a public media URL. Get clean local files back.

This repository is the open engine behind Puffy.

Use it if you want:

- a local-first CLI
- a localhost API
- a small Python client for scripts and workflows
- a Docker entrypoint you can self-host

If you just want a desktop installer, go to [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases).

## What This Repo Is Good For

This repo is for people who want to automate media-to-text work without hand-gluing tools together every week.

You can use it to:

- hand one public URL to a local pipeline
- get back files your agent, RAG flow, or notes app can reuse
- run Puffy as a tiny localhost service
- build your own workflow on top of a simple, inspectable surface

Typical users:

- CLI users
- local API users
- Python workflow builders
- n8n / Dify / Open WebUI tinkerers
- contributors who care about engine behavior and runtime policy

## What Stays Out Of This Repo

This repository is intentionally not the full desktop product.

It does not contain:

- the desktop GUI shell
- updater logic
- crash reporting or telemetry
- private release plumbing
- runtime binaries that have not passed public checks

## Start In 60 Seconds

### Run it locally

```bash
cargo run -p puffy-cli -- doctor
cargo run -p puffy-cli -- serve
curl http://127.0.0.1:41480/api/health
```

### Plan one extraction job

```bash
cargo run -p puffy-cli -- extract "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

With an explicit profile and output directory:

```bash
cargo run -p puffy-cli -- extract \
  "https://www.youtube.com/watch?v=dQw4w9WgXcQ" \
  --profile audio-only \
  --save-dir "$HOME/Documents/Puffy"
```

### Run it with Docker

```bash
docker build -t puffy-engine .
docker run --rm -p 41480:41480 puffy-engine
```

The container starts with `puffy serve` by default.

## What It Works With

The current source normalization covers:

- YouTube
- TikTok
- Douyin
- Bilibili
- X / Twitter
- Xiaohongshu
- Kuaishou
- Instagram

Host aliases such as `x.com` are normalized automatically.

## Local API

The API is intentionally small:

- `GET /health`
- `GET /api/health`
- `POST /api/extract`
- `GET /api/jobs/{job_id}`
- `GET /api/assets`
- `GET /api/search`

### Health check

```bash
curl http://127.0.0.1:41480/api/health
```

Example response:

```json
{
  "status": "ok",
  "name": "Puffy Public Engine",
  "version": "0.1.0-rehearsal"
}
```

### Queue one job

```bash
curl -X POST http://127.0.0.1:41480/api/extract \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'
```

Example response:

```json
{
  "status": "queued",
  "jobId": "asset-job-1743040100000-12345",
  "message": "Asset job queued. Poll /api/jobs/{job_id} for progress.",
  "checkUrl": "/api/jobs/asset-job-1743040100000-12345"
}
```

### Poll job state

```bash
curl http://127.0.0.1:41480/api/jobs/asset-job-1743040100000-12345
```

### Search or list assets

```bash
curl "http://127.0.0.1:41480/api/assets?limit=10"
curl "http://127.0.0.1:41480/api/search?q=knowledge+asset&limit=5"
```

## Python Client

The small Python client lives in [`clients/python`](./clients/python).

Install it locally:

```bash
cd clients/python
pip install -e .
```

Quick example:

```python
from puffy_client import PuffyClient

client = PuffyClient()
health = client.health()
accepted = client.extract_async("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
job = client.wait_for_job(accepted.job_id, poll_interval=1.0, timeout=30.0)
hits = client.search("knowledge asset", limit=5)
```

## What You Get Back

By default, Puffy plans a local asset directory under `~/Documents/Puffy`.

The target output shape includes:

- `video.mp4` or `audio.m4a`
- `transcript.txt`
- `transcript.md`
- `subtitle.srt`
- `subtitle.vtt`
- `segments.json`
- `chunks.jsonl`
- `chapters.json`

That is the part downstream tools should depend on: clean local files, not a desktop shell detail.

## Runtime Policy

The public engine is strict about runtime tools.

Right now this repo does not ship bundled `yt-dlp`, `ffmpeg`, or `ffprobe` in the public tree.
They only come back after their version, source, checksum, signing, and smoke status are all locked down.

Read the public runtime files here:

- [`runtime/runtime.lock.json`](./runtime/runtime.lock.json)
- [`runtime/README.md`](./runtime/README.md)

## Repo Map

```text
.
├── crates/
│   ├── puffy-core/
│   └── puffy-cli/
├── clients/
│   └── python/
├── docker/
├── examples/
└── runtime/
```

## Where To Go Next

- examples: [`examples/`](./examples)
- examples map: [`examples/README.md`](./examples/README.md)
- self-host notes: [`docker/selfhosted.md`](./docker/selfhosted.md)
- runtime policy: [`runtime/README.md`](./runtime/README.md)
- public installers: [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases)

## License

MIT. See [`LICENSE`](./LICENSE).
