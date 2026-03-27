from __future__ import annotations

import json
import time
from dataclasses import asdict
from typing import Any, Dict, Iterable, Optional
from urllib import error, parse, request

from .exceptions import PuffyHTTPError, PuffyTransportError
from .models import (
    AssetHit,
    AssetListResponse,
    HealthResponse,
    JobInfo,
    JobResponse,
    QueuedJobResponse,
    SearchResponse,
)


class PuffyClient:
    def __init__(self, base_url: str = "http://127.0.0.1:41480", timeout: float = 30.0):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout

    def health(self) -> HealthResponse:
        data = self._get_json("/api/health")
        return HealthResponse(
            status=data.get("status", "unknown"),
            name=data.get("name", "Puffy"),
            version=data.get("version", ""),
        )

    def extract(self, url: str, *, save_dir: Optional[str] = None, profile: Optional[str] = None):
        payload = {"url": url}
        if save_dir:
            payload["saveDir"] = save_dir
        if profile:
            payload["downloadProfile"] = profile
        data = self._post_json("/api/extract", payload)
        return QueuedJobResponse(
            status=data.get("status", "unknown"),
            job_id=data.get("jobId", ""),
            message=data.get("message", ""),
            check_url=data.get("checkUrl", ""),
        )

    def extract_async(self, url: str, *, save_dir: Optional[str] = None, profile: Optional[str] = None):
        return self.extract(url, save_dir=save_dir, profile=profile)

    def job(self, job_id: str) -> JobResponse:
        data = self._get_json(f"/api/jobs/{job_id}")
        job = data.get("job", {})
        return JobResponse(
            status=data.get("status", "unknown"),
            job=JobInfo(job_id=job.get("jobId", job_id), status=job.get("status", "")),
        )

    def wait_for_job(
        self,
        job_id: str,
        *,
        poll_interval: float = 1.0,
        timeout: float = 300.0,
    ) -> JobResponse:
        deadline = time.monotonic() + timeout
        while True:
            result = self.job(job_id)
            if result.job.status in {"success", "error"}:
                return result
            if time.monotonic() >= deadline:
                raise TimeoutError(f"Timed out waiting for job {job_id}")
            time.sleep(poll_interval)

    def assets(self, *, status: Optional[str] = None, q: Optional[str] = None, limit: int = 20) -> AssetListResponse:
        query = {"limit": str(limit)}
        if status:
            query["status"] = status
        if q:
            query["q"] = q
        data = self._get_json("/api/assets", query)
        return AssetListResponse(status=data.get("status", "unknown"), assets=data.get("assets", []))

    def search(self, q: str, *, limit: int = 10, status: Optional[str] = None) -> SearchResponse:
        query = {"q": q, "limit": str(limit)}
        if status:
            query["status"] = status
        data = self._get_json("/api/search", query)
        hits = [
            AssetHit(
                asset_id=item.get("assetId", ""),
                matched_field=item.get("matchedField", ""),
                excerpt=item.get("excerpt", ""),
            )
            for item in data.get("hits", [])
        ]
        return SearchResponse(status=data.get("status", "unknown"), hits=hits)

    def batch_extract(self, urls: Iterable[str]):
        return [self.extract_async(url) for url in urls]

    def _get_json(self, path: str, query: Optional[Dict[str, str]] = None) -> Dict[str, Any]:
        url = self._build_url(path, query)
        return self._request_json(url, method="GET")

    def _post_json(self, path: str, payload: Dict[str, Any]) -> Dict[str, Any]:
        url = self._build_url(path)
        body = json.dumps(payload).encode("utf-8")
        return self._request_json(url, method="POST", body=body)

    def _build_url(self, path: str, query: Optional[Dict[str, str]] = None) -> str:
        url = f"{self.base_url}{path}"
        if query:
            url = f"{url}?{parse.urlencode(query)}"
        return url

    def _request_json(
        self,
        url: str,
        *,
        method: str,
        body: Optional[bytes] = None,
    ) -> Dict[str, Any]:
        req = request.Request(
            url,
            data=body,
            headers={"Content-Type": "application/json"},
            method=method,
        )
        try:
            with request.urlopen(req, timeout=self.timeout) as response:
                payload = response.read().decode("utf-8")
                return json.loads(payload)
        except error.HTTPError as exc:
            body = exc.read().decode("utf-8", errors="replace")
            raise PuffyHTTPError(exc.code, body) from exc
        except (error.URLError, json.JSONDecodeError, OSError) as exc:
            raise PuffyTransportError(str(exc)) from exc

