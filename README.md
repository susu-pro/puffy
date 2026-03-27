# Puffy Public Engine

Puffy turns public media into reusable local assets.

This repository is the public engine lane for technical users:

- local-first CLI
- localhost headless API
- Python SDK for workflow tooling
- Docker and self-hosted entrypoints
- fixed runtime contract for public distribution

If you want desktop installers instead of source or CLI flows, use [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases).

For this repository, the rule is simple:

- the engine stays public
- the GUI shell stays out of this repository
- runtime binaries stay external until they meet the lock, signing, and smoke gates

## Why This Repo Exists

Puffy is not trying to be another pile of one-off download scripts.

The public engine lane exists so you can:

- feed one public URL into a stable local pipeline
- get back files that agents, RAG jobs, and note systems can reuse
- self-host a small localhost ingress instead of wiring `yt-dlp`, `ffmpeg`, subtitles, and transcript cleanup by hand
- build your own workflows on top of a narrow, inspectable contract

This repository is for the technical surface of Puffy:

- CLI operators
- local API users
- workflow builders using Python, n8n, Dify, or Open WebUI
- contributors who care about engine behavior, runtime policy, and output contracts

## What This Repo Contains

### Public surface

- `crates/puffy-core`: engine planning, source normalization, runtime policy, structured output contract
- `crates/puffy-cli`: `puffy` binary with `doctor`, `serve`, `extract`, and `runtime`
- `clients/python`: tiny Python client for local automation
- `examples/`: starter integrations for agent and workflow tools
- `docker/`: self-hosted entrypoint docs
- `runtime/`: runtime lock and public distribution rules

### Intentionally out of scope

- desktop GUI shell
- updater logic
- crash reporting and telemetry
- private release plumbing
- bundled runtime binaries that have not passed public gates

## Quick Start

### Build and run locally

```bash
cargo run -p puffy-cli -- doctor
cargo run -p puffy-cli -- serve
curl http://127.0.0.1:41480/api/health
```

### Plan one extraction job from the CLI

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

### Run with Docker

```bash
docker build -t puffy-engine .
docker run --rm -p 41480:41480 puffy-engine
```

The container entrypoint defaults to `puffy serve`.

## Supported Public Sources

The current public source normalization covers:

- YouTube
- TikTok
- Douyin
- Bilibili
- X / Twitter
- Xiaohongshu
- Kuaishou
- Instagram

The engine normalizes host aliases such as `x.com` to the canonical route it expects internally.

## API Contract

The public engine exposes a small localhost API:

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

### Queue one extract job

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

### Search or list

```bash
curl "http://127.0.0.1:41480/api/assets?limit=10"
curl "http://127.0.0.1:41480/api/search?q=knowledge+asset&limit=5"
```

## Python SDK

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

## Output Contract

Each job plans a local asset directory under `~/Documents/Puffy` by default.

The target output contract includes:

- `video.mp4` or `audio.m4a`
- `transcript.txt`
- `transcript.md`
- `subtitle.srt`
- `subtitle.vtt`
- `segments.json`
- `chunks.jsonl`
- `chapters.json`

This contract is what downstream tools should depend on, not a desktop shell implementation detail.

## Runtime Rules

The runtime contract for the public engine is intentionally strict.

This repository does not currently ship bundled `yt-dlp`, `ffmpeg`, or `ffprobe` binaries in the public engine tree.
Until a runtime is fixed, hashed, signed, and smoke-tested for public distribution, it stays external.

The public lock lives here:

- [`runtime/runtime.lock.json`](./runtime/runtime.lock.json)

The public rules live here:

- [`runtime/README.md`](./runtime/README.md)

Public distribution may only re-introduce bundled runtime binaries after all of the following pass:

- fixed version
- source URL recorded
- sha256 recorded
- signing identity recorded
- smoke verification recorded

## Repo Layout

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

## Examples

Starter integrations live in [`examples/`](./examples):

- `langchain_video_rag.py`
- `n8n_puffy_async_template.json`
- `dify_puffy_video_summary.yml`
- `open_webui_local_research.md`

See [`examples/README.md`](./examples/README.md) for the short map.

## Distribution Model

The public engine repository is not the desktop distribution channel.

- source, CLI, API, examples, and runtime policy live here
- public installers live in [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases)

That split is intentional. It keeps the engine open, scriptable, and contribution-friendly without dragging shell-specific release baggage back into the public repository.

## License

MIT. See [`LICENSE`](./LICENSE).
