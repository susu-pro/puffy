# Runtime Rules

This directory documents the public engine runtime contract.

Current rehearsal policy:

- do not ship bundled `yt-dlp`, `ffmpeg`, or `ffprobe`
- record only version, sha256, origin, codesign, notarization, and smoke status
- require fixed version plus post-sign smoke before re-entering public distribution

## Override Path

During local development, explicit environment variables may point to local tools.
The published public engine should still treat runtime tools as external assets
until a fixed, signed, smoke-tested bundle exists.

