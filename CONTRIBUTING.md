# Contributing

Thanks for helping build VegieBreakDAW.

## Before you start

Read [`docs/CLEANROOM.md`](docs/CLEANROOM.md). The short version:

- Do not copy FL Studio code, decompiled output, assets, presets, samples, or
  strings.
- Do not add license bypasses, unlockers, or activation patches.
- Implement from written behavior specifications and tests.
- Keep third-party licenses compatible and documented.

## Development

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p vb-app -- --doctor
```

On Windows, MSVC is recommended. If Visual Studio Build Tools are unavailable,
the workspace also builds with `stable-x86_64-pc-windows-gnu` and MinGW-w64
UCRT. Native plugin SDK work may require MSVC.

## Pull requests

- Keep changes focused.
- Add tests for behavior, not just code coverage.
- Update `CHANGELOG.md` and the README change-log table for user-visible work.
- Record new third-party dependencies in `docs/THIRD_PARTY.md`.
- Do not commit build output, models, audio, or FL Studio binaries.

## Commit messages

Use an imperative subject, for example:

```
audio: add deterministic block transport
core: validate mixer sends
docs: record FL Studio plugin layout
```

## Reporting security issues

Do not open a public issue for a vulnerability that could affect users. Contact
the maintainers privately and include a minimal reproduction.
