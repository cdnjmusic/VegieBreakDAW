# VegieBreakDAW

**A clean-room, open-source, FL-inspired digital audio workstation written in Rust.**

VegieBreakDAW is an independent project. It is **not** affiliated with, endorsed by, or
derived from Image-Line's FL Studio. It contains no Image-Line source code, binaries,
artwork, presets, samples, or license-circumvention functionality.

The goals are simple:

- **No account. No sign-in. No telemetry. No cloud requirement.**
- **All of our own built-in devices are included by default.** No unlocks, no tiers.
- **First-class VST3 and CLAP hosting**, with an optional AAX backend where the user
  supplies Avid's SDK under Avid's license.
- **A real stem extractor** for vocals, drums, bass, and other.
- **A fast Rust audio engine**: 64-bit float, preallocated buffers, lock-free message
  passing, and a zero-allocation audio callback.
- **An FL-style workflow** (channel rack, step sequencer, piano roll, playlist, mixer)
  with original visual design.

> **What we will not do:** bypass FL Studio's licensing, unlock Image-Line plugins, or
> copy protected code, DSP, presets, artwork, or sample content. FL Studio stays a
> separate licensed product. This project is an interoperability-focused clean-room
> reimplementation of workflow ideas, not a redistribution of FL Studio.

## Current status

Phase 0 is complete: the Rust workspace, analysis toolchain, and repository are
bootstrapped. The audio engine, plugin host, and stem extractor currently expose
interfaces and validation scaffolding; they are not yet production components.

| Area | Status |
| --- | --- |
| Rust workspace and CI | ✅ scaffolded |
| Project model, save/load, validation | ✅ implemented |
| Audio transport and null backend | ✅ implemented |
| WASAPI/ASIO device layer | ⏳ planned for Phase 1 |
| VST3/CLAP hosting | ⏳ interface only; Phase 3 |
| AAX hosting | ⛔ optional, requires Avid SDK |
| Stem separation | ⏳ interface + model manifest; Phase 2 |
| Built-in original devices | ⏳ Phase 5 |
| UI shell | ⏳ Phase 1 |

## Quick start

### Windows prerequisites

- Rust stable. **MSVC is the recommended host**; the GitHub CI uses
  `stable-x86_64-pc-windows-msvc`.
- Visual Studio Build Tools 2022 with **Desktop development with C++** (MSVC).
- CMake and Ninja (for native dependencies).
- Git.

**Local fallback:** the Visual Studio installer on this machine could not fetch
its channel catalog, so the workspace is currently verified with
`stable-x86_64-pc-windows-gnu` and MinGW-w64 UCRT at `C:\Tools\mingw64`.
Both hosts compile the same crates; native plugin SDKs may still require MSVC
later.

### Build and test

```powershell
cargo build --workspace
cargo test --workspace
cargo run -p vb-app -- --doctor
```

Create a project without opening a UI:

```powershell
cargo run -p vb-app -- --new demo.vbproj
cargo run -p vb-app -- --inspect demo.vbproj
```

## Architecture at a glance

| Crate | Responsibility |
| --- | --- |
| `vb-core` | project model, tempo/time conversion, validation |
| `vb-audio` | device abstraction, transport, mixer graph, DSP scheduling |
| `vb-ui` | GPU-accelerated shell, timeline and piano-roll rendering |
| `vb-host` | VST3/CLAP discovery, loading, state, process isolation |
| `vb-stems` | HTDemucs/ONNX stem separation and progress reporting |
| `vb-io` | WAV/FLAC/MIDI decoding, project persistence, export |
| `vb-app` | application entry point, diagnostics, crash recovery |

The detailed design, real-time rules, and performance budgets live in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md). The clean-room rules are in
[`docs/CLEANROOM.md`](docs/CLEANROOM.md). The delivery plan is in
[`docs/ROADMAP.md`](docs/ROADMAP.md).

## Analysis toolchain

This project uses [REA](https://github.com/morluto/rea) for clean-room,
interoperability-focused analysis of the installed FL Studio build. REA is connected to
the DeepSeek Harness session through its MCP server.

The current machine has:

- FL Studio 2026 `26.1.7.5654` at `C:\Program Files\Image-Line\FL Studio 2026`
- Ghidra `12.1.4` at `C:\Tools\ghidra_12.1.4_PUBLIC`
- Temurin JDK 21
- REA `6.7.0` with the `rea` MCP registration in `C:\Users\Potaku\.dsh\cordis.patch.yml`

See [`docs/REA.md`](docs/REA.md) for the exact setup, verification commands, and
analysis scope.

## Roadmap

1. **Audio and stem vertical slice** — device, WAV playback, mixer, 4-stem separation,
   solo/mute, stem export.
2. **Sequencer** — channel rack, step sequencer, piano roll, playlist, patterns, MIDI.
3. **Plugin host** — VST3 + CLAP scan/load/editor/state, crash isolation, PDC.
4. **Recording and export** — audio/MIDI recording, offline render, stem export.
5. **Built-in devices** — original synth, sampler, drums, EQ, compressor, reverb,
   delay, limiter.
6. **Performance and packaging** — 64-sample buffer, SIMD, installer, portable build.

## Change log

The full, versioned history is in [`CHANGELOG.md`](CHANGELOG.md). The short version:

| Date | Change |
| --- | --- |
| 2026-10-21 | Bootstrap: Rust workspace, core project model, audio transport/null backend, plugin/stem/UI interfaces, CI, GPL-3.0 license, clean-room docs, REA + Ghidra setup, initial git commit. |

## License

VegieBreakDAW is free software: you can redistribute it and/or modify it under the
terms of the **GNU General Public License, version 3 or later** (GPL-3.0-or-later).
See [`LICENSE`](LICENSE).

The GPL choice keeps the project compatible with the VST3 SDK's open-source option.
Individual crates may carry permissive licenses where they do not link the VST3 SDK;
the combined application is GPL-3.0-or-later.

Third-party components and model licenses are recorded in
[`docs/THIRD_PARTY.md`](docs/THIRD_PARTY.md).

## Contributing

Please read [`docs/CLEANROOM.md`](docs/CLEANROOM.md) before contributing. In short:

- Do not paste decompiled code, FL Studio assets, or proprietary constants.
- Record observable behavior and formats as evidence.
- Implement from the written specification, not from copied source.
- Never add license bypasses, unlockers, or cracked plugin loaders.
