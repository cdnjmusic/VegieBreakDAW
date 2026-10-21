# Roadmap

The project is built in vertical slices. Each phase ends with a runnable artifact
and measurable acceptance criteria.

## Phase 0 — Bootstrap (complete, 2026-10-21)

- Rust workspace, CI, license, ignore rules.
- Project model with save/load and validation.
- Transport and null audio backend.
- Plugin, stem, UI, and I/O interfaces.
- REA `6.7.0` connected to DeepSeek Harness; Ghidra `12.1.4` + JDK 21 verified.
- Clean-room policy and FL Studio evidence plan.

**Acceptance:** `cargo test --workspace` passes; `rea doctor --client dsh --skill`
reports healthy; Ghidra provider opens `FL64.exe` read-only.

## Phase 1 — Audio and stem vertical slice

- WASAPI shared/exclusive device; optional ASIO with user-supplied SDK.
- Lock-free command ring; preallocated graph; 64/128-sample blocks.
- WAV/FLAC import, waveform peaks, transport, master gain.
- 4-stem ONNX separation with progress and cancellation.
- Stem playback with solo/mute and stem export.
- Minimal UI: browser, transport, channel list, waveform.

**Acceptance:** 3-minute track separates into 4 aligned stems; 10-minute playback
at 64 samples with no xruns; UI stays responsive during separation.

## Phase 2 — Sequencer and arrangement

- Channel rack and step sequencer.
- Piano roll with note create/move/resize, velocity, slide, and scale helpers.
- Playlist with pattern and audio clips, loop regions, and snap.
- Patterns, automation lanes, and MIDI import/export.
- Undo/redo across all edit operations.

**Acceptance:** build a 16-bar arrangement with 8 patterns and 16 channels; save,
reload, and export a MIDI file with correct timing.

## Phase 3 — Plugin host

- VST3 and CLAP scanning with a persistent cache.
- Out-of-process hosting, shared-memory audio, watchdog, auto-bypass.
- Plugin editors as native windows; state save/restore.
- PDC, plugin delay reporting, and per-slot latency display.
- AAX backend interface (not distributed; requires Avid SDK).

**Acceptance:** scan 100+ plugins without blocking the UI; host a free VST3
instrument and a CLAP effect; kill the plugin process and confirm the project
continues with the slot bypassed.

## Phase 4 — Recording and export

- Audio input recording with monitoring and latency compensation.
- MIDI recording and quantization.
- Offline render: master, stems, selected region, and individual tracks.
- Export formats: WAV 16/24/32-bit float, FLAC.

**Acceptance:** record a 4-bar audio loop; render a 2-minute project offline
faster than real time; exported stems sum to the master within -120 dBFS.

## Phase 5 — Built-in original devices

- Polymorphic synth with wavetable and subtractive engines.
- Sampler with slicing and time-stretch.
- Drum machine with synthesis and sample layers.
- EQ, compressor, reverb, delay, chorus, limiter, and utility.
- Original presets only.

**Acceptance:** a factory demo project uses only built-in devices; CPU stays under
25% on the reference machine at 128 samples.

## Phase 6 — Performance and packaging

- SIMD kernels and denormal-safe DSP.
- Graph parallelism for independent buses.
- Startup and project-load profiling.
- Windows installer, portable ZIP, and signed release artifacts.
- Accessibility pass: keyboard navigation, screen-reader labels, scalable UI.

**Acceptance:** cold start < 2 s; 100-track project at 64 samples with < 60% CPU;
installer and portable build both pass a clean-machine smoke test.

## Explicit non-goals

- Reproducing FL Studio's proprietary DSP or factory content.
- Loading FL Studio's bundled plugins without a valid license.
- Bypassing activation, sign-in, or plugin unlocks.
- Shipping the AAX SDK or VST3 SDK under terms we cannot satisfy.
- Pixel-identical UI cloning or trademark use.
