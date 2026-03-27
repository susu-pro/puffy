# puffy-client

Small Python SDK for the Puffy public engine.

## Install

```bash
pip install -e .
```

## Quick Start

```python
from puffy_client import PuffyClient

client = PuffyClient()
health = client.health()
print(health.version)

accepted = client.extract_async("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
print(accepted.job_id)
```

## Async Flow

```python
from puffy_client import PuffyClient

client = PuffyClient()
accepted = client.extract_async("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
job = client.wait_for_job(accepted.job_id, poll_interval=1.0, timeout=30.0)
print(job.job.status)
```

## Search

```python
from puffy_client import PuffyClient

client = PuffyClient()
hits = client.search("knowledge asset", limit=5)
print(len(hits.hits))
```

