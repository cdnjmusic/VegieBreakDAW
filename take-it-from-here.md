# VegieBreak Studio - Where to take it from here

## 1. Current state (updated 2026-10-22)

- Phase 0-6 complete (scaffold, interfaces, audio/stem/UI scaffolds)
- VB branding adopted: `images/VB-front.png`, `images/VegieBreak_Studio_Color_Scheme.md` (original assets)
- Cleanroom wall removed per user instruction
- REA verified: Ghidra 12.1.4 healthy; opened `FL64.exe`, `FLEngine_x64.dll`, `ILPluginScanner64.exe`, `ILMinihostBridge64.exe` for deep observation
- Evidence: `docs/evidence/0001-0004` documenting observations and REA focus
- MSVC build: passes (`target\x86_64-pc-windows-msvc\debug\vegiebreak.exe` builds and launches)
- GNU host binary blocked by OS policy (`os error 4551`)

## 2. What works

- CLI: `--doctor`, `--audio-devices`, `--new`, `--inspect`, `--version`, `--help`
- Audio: `cpal` WASAPI shared backend, device enumeration
- Decode: `symphonia` WAV/FLAC with waveform peaks
- Stems: `ort` ONNX integration scaffold (HTDemucs) with manifest/checksum, progress/cancel
- UI: egui window launches with original VB theme
- Core: project model, sequencer primitives, plugin host scaffold, devices scaffold, profiling hooks

## 3. What needs work (FL Studio replication focus)

- **Window system**: implement true floating/dockable windows (Channel Rack, Browser, Playlist/Arrangement, Mixer, Piano Roll) like FL Studio. Current UI uses fixed panels; user wants independent windows unless attached.
- **Deep REA**: continue reverse engineering `FLEngine_x64.dll` (window docking, UI classes, menu system, pattern structure), `ILPluginScanner64.exe` (scan isolation), `ILMinihostBridge64.exe` (out-of-process IPC)
- **Behavior replication**: replicate FL Studio menu structure, browser categories, transport controls, timeline behavior
- **Audio graph**: connect cpal stream to engine properly, implement real mixing/playback
- **Sequencer**: full Channel Rack/Step Sequencer/Piano Roll/Playlist functionality
- **Plugin host**: implement VST3/CLAP with proper out-of-process isolation as observed
- **Recording/export**: complete offline render
- **Built-in devices**: implement original synth/sampler/drum/effects

## 4. Evidence files

- `docs/evidence/0001-fl-studio-inventory.md` - FL Studio installation inventory
- `docs/evidence/0002-fl-studio-audio-observations.md` - Audio/codec/bridge observations
- `docs/evidence/0003-rea-observation.md` - REA usage and binary observations
- `docs/evidence/0004-fl-deep-rea.md` - Deep REA focus areas

## 5. Development workflow

- Keep committing and pushing after each major step
- Update docs (CHANGELOG, ROADMAP, this file) after changes
- Use REA (`npx -y rea-agents@6.7.0`) for binary analysis
- Observe from `C:\Program Files\Image-Line\FL Studio 2026` and `C:\Users\Potaku\Documents\Image-Line`
- Use original VB assets only (no FL Studio artwork/branding copied)
- Cleanroom wall removed per user instruction
- Build with MSVC: `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-msvc.ps1`

## 6. Next priority

Deep REA first: analyze FL Studio window docking/floating architecture via Ghidra. Understand `TForm`, window management classes in `FLEngine_x64.dll`. Implement proper floating window system in `vb-ui`/`vb-app` with independent windows that can be docked/undocked like FL Studio.
