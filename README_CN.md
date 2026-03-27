# Puffy 公共引擎

Puffy 把公开视频变成本地可复用资产。

这个仓是 Puffy 面向技术用户的公开引擎线：

- 本地优先 CLI
- localhost headless API
- 面向工作流工具的 Python SDK
- Docker 与自托管入口
- 面向公开分发的 runtime 固定规则

如果你只是想直接下载安装包，而不是自己跑源码或 CLI，请去 [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases)。

这个仓的边界很明确：

- 引擎保持公开
- GUI 壳不放进这个仓
- runtime 二进制在通过 lock、签名、smoke gate 之前，不回到公开分发

## 这个仓为什么存在

Puffy 不是一堆一次性下载脚本的拼装体。

公开引擎线的目标是：

- 给一个公开 URL，产出一组稳定的本地资产
- 让 agent、RAG、笔记系统能直接复用这些文件
- 提供一个可自托管的本地入口，替代手工拼 `yt-dlp`、`ffmpeg`、字幕和 transcript 清洗链
- 让工作流开发者基于一个窄而清晰的合同面构建自己的系统

这个仓主要服务：

- CLI 用户
- 本地 API 接入方
- Python / n8n / Dify / Open WebUI 工作流搭建者
- 关心引擎行为、runtime 政策、输出合同的贡献者

## 这个仓包含什么

### 公开能力面

- `crates/puffy-core`：引擎规划、来源规范化、runtime 政策、结构化输出合同
- `crates/puffy-cli`：`puffy` 二进制，提供 `doctor`、`serve`、`extract`、`runtime`
- `clients/python`：小型 Python 客户端，方便本地自动化接入
- `examples/`：面向 agent 和工作流工具的起步示例
- `docker/`：自托管入口与文档
- `runtime/`：runtime lock 与公开分发规则

### 故意不放进来的东西

- 桌面 GUI 壳
- updater
- crash reporting / telemetry
- 私有发版链路
- 尚未通过公开门禁的内置 runtime 二进制

## 快速开始

### 本地运行

```bash
cargo run -p puffy-cli -- doctor
cargo run -p puffy-cli -- serve
curl http://127.0.0.1:41480/api/health
```

### 用 CLI 规划一个提取任务

```bash
cargo run -p puffy-cli -- extract "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

指定 profile 和输出目录：

```bash
cargo run -p puffy-cli -- extract \
  "https://www.youtube.com/watch?v=dQw4w9WgXcQ" \
  --profile audio-only \
  --save-dir "$HOME/Documents/Puffy"
```

### 用 Docker 跑

```bash
docker build -t puffy-engine .
docker run --rm -p 41480:41480 puffy-engine
```

容器入口默认就是 `puffy serve`。

## 当前支持的公开来源

目前公开来源规范化覆盖：

- YouTube
- TikTok
- Douyin
- Bilibili
- X / Twitter
- 小红书
- 快手
- Instagram

引擎内部会把 `x.com` 这类 host alias 归一到它预期的规范地址。

## API 合同

公共引擎目前暴露一组很小的 localhost API：

- `GET /health`
- `GET /api/health`
- `POST /api/extract`
- `GET /api/jobs/{job_id}`
- `GET /api/assets`
- `GET /api/search`

### 健康检查

```bash
curl http://127.0.0.1:41480/api/health
```

示例返回：

```json
{
  "status": "ok",
  "name": "Puffy Public Engine",
  "version": "0.1.0-rehearsal"
}
```

### 提交一个提取任务

```bash
curl -X POST http://127.0.0.1:41480/api/extract \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'
```

示例返回：

```json
{
  "status": "queued",
  "jobId": "asset-job-1743040100000-12345",
  "message": "Asset job queued. Poll /api/jobs/{job_id} for progress.",
  "checkUrl": "/api/jobs/asset-job-1743040100000-12345"
}
```

### 轮询任务状态

```bash
curl http://127.0.0.1:41480/api/jobs/asset-job-1743040100000-12345
```

### 搜索或列资产

```bash
curl "http://127.0.0.1:41480/api/assets?limit=10"
curl "http://127.0.0.1:41480/api/search?q=knowledge+asset&limit=5"
```

## Python SDK

小型 Python 客户端在 [`clients/python`](./clients/python)。

本地安装：

```bash
cd clients/python
pip install -e .
```

快速示例：

```python
from puffy_client import PuffyClient

client = PuffyClient()
health = client.health()
accepted = client.extract_async("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
job = client.wait_for_job(accepted.job_id, poll_interval=1.0, timeout=30.0)
hits = client.search("knowledge asset", limit=5)
```

## 输出合同

每个任务默认会在 `~/Documents/Puffy` 下规划一个本地资产目录。

目标输出合同包括：

- `video.mp4` 或 `audio.m4a`
- `transcript.txt`
- `transcript.md`
- `subtitle.srt`
- `subtitle.vtt`
- `segments.json`
- `chunks.jsonl`
- `chapters.json`

下游工具应该依赖的是这套输出合同，而不是桌面壳的实现细节。

## Runtime 规则

公共引擎对 runtime 的规则是故意收紧的。

当前这个公开引擎树不会直接分发内置 `yt-dlp`、`ffmpeg`、`ffprobe`。
在 runtime 没有完成固定版本、哈希、签名、smoke 验证之前，它就保持外置。

公开 lock 在：

- [`runtime/runtime.lock.json`](./runtime/runtime.lock.json)

公开规则说明在：

- [`runtime/README.md`](./runtime/README.md)

只有以下条件全部满足后，bundled runtime 才允许重新进入公开分发：

- 固定版本
- 记录来源 URL
- 记录 sha256
- 记录签名身份
- 记录 smoke 验证结果

## 仓结构

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

## 示例

起步示例在 [`examples/`](./examples)：

- `langchain_video_rag.py`
- `n8n_puffy_async_template.json`
- `dify_puffy_video_summary.yml`
- `open_webui_local_research.md`

简要说明见 [`examples/README.md`](./examples/README.md)。

## 分发模型

公开引擎仓不是桌面版分发通道。

- 源码、CLI、API、示例、runtime 政策放在这里
- 公开安装包统一放在 [`susu-pro/puffy-download`](https://github.com/susu-pro/puffy-download/releases)

这个分离是故意的。这样公开仓可以继续保持开源、可脚本化、可贡献，而不会把桌面壳特有的发版包袱重新拖回来。

## 许可证

MIT，见 [`LICENSE`](./LICENSE)。
