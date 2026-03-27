# Puffy Public Engine

Puffy turns public media into reusable local assets.

This repository is the public engine lane:

- a local-first CLI and headless API
- structured transcript and sidecar generation
- Python SDK examples for agent workflows
- Docker and self-hosted entrypoints

## Quick Start

```bash
cargo run -p puffy-cli -- doctor
cargo run -p puffy-cli -- serve
curl http://127.0.0.1:41480/api/health
```

## What It Produces

Each job plans a local asset directory with:

- `video.mp4` or `audio.m4a`
- `transcript.txt`
- `transcript.md`
- `subtitle.srt`
- `subtitle.vtt`
- `segments.json`
- `chunks.jsonl`
- `chapters.json`

## Runtime Rules

This cutover rehearsal does not ship bundled runtime binaries.
The runtime lock records fixed versions, sha256, and smoke status only.

## Downloads

Public downloads live in `susu-pro/puffy-download`.
This repo only owns the engine, CLI, and headless API surface.
