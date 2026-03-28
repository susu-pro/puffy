<h1 align="center" id="readme-top">
  Puffy Public Engine
</h1>

<p align="center">
  <b>Paste a public media URL. Get clean local files back.</b><br>
  Open CLI + local API + Python client for people who want a scriptable engine,<br>
  not another week of gluing tools together by hand.
</p>

<p align="center">
  <a href="https://github.com/susu-pro/puffy/stargazers"><img src="https://img.shields.io/github/stars/susu-pro/puffy?style=for-the-badge&color=f0c674" alt="Stars" /></a>
  <img src="https://img.shields.io/badge/CLI_%2B_API-local--first-black?style=for-the-badge" alt="CLI and API" />
  <img src="https://img.shields.io/badge/Docker-self--hosted-blue?style=for-the-badge" alt="Docker" />
  <a href="https://github.com/susu-pro/puffy/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="MIT License" /></a>
</p>

<p align="center">
  <img src="./public/demo-cli.gif" width="880" alt="Puffy CLI Demo" />
</p>

<p align="center">
  <a href="#get-started"><strong>Get Started</strong></a> ·
  <a href="https://github.com/susu-pro/puffy-download/releases/download/v0.1.5/Puffy_0.1.5_aarch64.dmg"><strong>Download Mac</strong></a> ·
  <a href="https://github.com/susu-pro/puffy-download/releases/download/v0.1.5/Puffy_0.1.5_x64-setup.exe"><strong>Download Windows</strong></a> ·
  <a href="#local-api"><strong>API</strong></a> ·
  <a href="#examples"><strong>Examples</strong></a> ·
  <a href="#output"><strong>Output</strong></a> ·
  <a href="#python-client"><strong>Python</strong></a> ·
  <a href="#runtime-policy"><strong>Runtime</strong></a> ·
  <a href="./README_CN.md">🇨🇳 简体中文</a>
</p>

---

## Direct Download

| Platform | Size | Direct Link |
| --- | --- | --- |
| macOS (Apple Silicon) | ~30 MB | [Download DMG](https://github.com/susu-pro/puffy-download/releases/download/v0.1.5/Puffy_0.1.5_aarch64.dmg) |
| macOS (Intel) | ~62 MB | [Download DMG](https://github.com/susu-pro/puffy-download/releases/download/v0.1.5/Puffy_0.1.5_x64.dmg) |
| Windows | ~21 MB | [Download Installer](https://github.com/susu-pro/puffy-download/releases/download/v0.1.5/Puffy_0.1.5_x64-setup.exe) |

This repository is the open engine behind Puffy.

Use it if you want:

- a local-first CLI
- a localhost API
- a small Python client for scripts and workflows
- a Docker entrypoint you can self-host

If you just want a desktop installer, use the direct download links above.

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

<a id="get-started"></a>

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

<a id="local-api"></a>

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

<a id="python-client"></a>

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

<a id="examples"></a>

## Examples

Starter integrations live in [`examples/`](./examples):

- `langchain_video_rag.py`: kick off a job, wait for completion, then load the output into a LangChain-style flow
- `n8n_puffy_async_template.json`: tiny async polling template for n8n
- `dify_puffy_video_summary.yml`: minimal Dify-style HTTP workflow sketch
- `open_webui_local_research.md`: use Puffy as a local ingest step for research workflows

See [`examples/README.md`](./examples/README.md) for the quick map.

<a id="output"></a>

## Output

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

<a id="runtime-policy"></a>

## Runtime Policy

The public engine is strict about runtime tools.

Right now this repo does not ship bundled `yt-dlp`, `ffmpeg`, or `ffprobe` in the public tree.
They only come back after their version, source, checksum, signing, and smoke status are all locked down.

Read the public runtime files here:

- [`runtime/runtime.lock.json`](./runtime/runtime.lock.json)
- [`runtime/README.md`](./runtime/README.md)

## Contributing

If you want to improve the public engine, start with the boring path:

```bash
git clone https://github.com/susu-pro/puffy.git && cd puffy
cargo test
cargo run -p puffy-cli -- doctor
cd clients/python && python -m unittest discover tests
```

Keep changes focused on the engine, CLI, Python client, examples, and runtime policy.

## Feedback

If you are trying Puffy in a real workflow, open an issue and tell us:

- what link you tried
- whether you used the CLI, local API, or Python client
- what file or result you expected to get next

That feedback is what decides the next batch of source support, DX fixes, and workflow improvements.

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

---

<a href="https://star-history.com/#susu-pro/puffy&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=susu-pro/puffy&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=susu-pro/puffy&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=susu-pro/puffy&type=Date" width="600" />
  </picture>
</a>
