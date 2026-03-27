# Open WebUI Local Research Pattern

Use the public engine as the media ingress step:

1. Queue the URL with `POST /api/extract`.
2. Poll the job with `GET /api/jobs/:id`.
3. Load `segments.json` or `chunks.jsonl` into your local research stack.

The public engine stays local and predictable, so the downstream agent can focus on retrieval and synthesis.

