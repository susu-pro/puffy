from dataclasses import dataclass, field
from typing import List, Optional


@dataclass
class HealthResponse:
    status: str
    name: str
    version: str


@dataclass
class QueuedJobResponse:
    status: str
    job_id: str
    message: str
    check_url: str


@dataclass
class JobInfo:
    job_id: str
    status: str


@dataclass
class JobResponse:
    status: str
    job: JobInfo


@dataclass
class AssetHit:
    asset_id: str
    matched_field: str = ""
    excerpt: str = ""


@dataclass
class AssetListResponse:
    status: str
    assets: List[dict] = field(default_factory=list)


@dataclass
class SearchResponse:
    status: str
    hits: List[AssetHit] = field(default_factory=list)

