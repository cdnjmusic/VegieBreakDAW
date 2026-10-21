# Changelog

All notable changes to VegieBreakDAW are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- WASAPI/ASIO device layer and real-time audio callback.
- WAV/FLAC playback and offline render.
- Stem separation vertical slice with HTDemucs ONNX.
- VST3/CLAP scanner and out-of-process plugin host.
- Original built-in devices.

## [0.1.0] - 2026-10-21

### Added

- Initial Cargo workspace with `vb-core`, `vb-audio`, `vb-ui`, `vb-host`,
  `vb-stems`, `vb-io`, and `vb-app`.
- Versioned project model with tracks, clips, patterns, notes, mixer channels,
  tempo, time signature, and validation.
- JSON project save/load and CLI `--new` / `--inspect` commands.
- Transport and a deterministic null audio backend for tests.
- Plugin-format descriptors for VST3, CLAP, VST2 (legacy), and AAX (SDK-gated).
- Stem-interface types and the MIT-licensed HTDemucs model manifest.
- `--doctor` diagnostics for platform, project schema, audio backend, plugin
  formats, and stem backend.
- Unit tests for project round-trips, validation, timing, transport, and stem
  labels.
- GitHub Actions CI for Windows and Linux.
- GPL-3.0-or-later license, `.gitignore`, `.editorconfig`, and `.gitattributes`.
- Clean-room policy, architecture, roadmap, REA setup, and third-party notices.
- REA `6.7.0` MCP registration for DeepSeek Harness and the installed
  `reverse-engineer-anything` skill.
- Ghidra `12.1.4` and Temurin JDK 21 analysis toolchain.
- Verified Ghidra import and overview on a compiled PE smoke target (145
  procedures, 267 strings).
- Local GNU-host build fallback (`stable-x86_64-pc-windows-gnu` + MinGW-w64
  UCRT) because the Visual Studio Build Tools channel download failed; CI
  remains MSVC.
- Installed LLVM 23.1.4 and Windows SDK 10.0.26100 for an independent MSVC ABI
  link path.
- Added `scripts/verify-setup.ps1` for read-only toolchain verification.
- Added `scripts/build-msvc.ps1`, which builds and tests the workspace for
  `x86_64-pc-windows-msvc` using LLVM `lld-link`, the portable MSVC CRT, and
  the Windows SDK.
- Verified `cargo test --workspace --target x86_64-pc-windows-msvc` and the
  MSVC-built `vegiebreak.exe` CLI.
- Published the initial repository and verification docs to
  <https://github.com/cdnjmusic/VegieBreakDAW>.
- Added [`take-it-from-here.md`](take-it-from-here.md) with the complete
  handoff, machine/toolchain state, verified commands, known blockers, and the
  Phase 1 audio + stem implementation plan.

### Security

- No account system, telemetry, analytics, or network calls in the application.
- No license-bypass or plugin-unlock functionality.
