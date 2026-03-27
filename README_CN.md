# Puffy 公共引擎

Puffy 把公开视频变成本地可复用资产。

这个仓只承担公开引擎线：

- 本地优先的 CLI 和 headless API
- 结构化 transcript 与 sidecar 生成
- 面向 agent 工作流的 Python SDK 示例
- Docker 和自托管入口

## 快速开始

```bash
cargo run -p puffy-cli -- doctor
cargo run -p puffy-cli -- serve
curl http://127.0.0.1:41480/api/health
```

## 输出内容

每个任务都会规划出一个本地资产目录，里面包含：

- `video.mp4` 或 `audio.m4a`
- `transcript.txt`
- `transcript.md`
- `subtitle.srt`
- `subtitle.vtt`
- `segments.json`
- `chunks.jsonl`
- `chapters.json`

## Runtime 规则

这份切仓预演不直接分发内置 runtime 二进制。
runtime lock 只记录固定版本、sha256 和 smoke 状态。

## 下载

公开下载入口统一放在 `susu-pro/puffy-download`。
这个仓只保留引擎、CLI 和 headless API。
