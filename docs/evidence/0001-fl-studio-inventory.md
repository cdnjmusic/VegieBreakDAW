# Evidence 0001 — FL Studio 2026 installation inventory

- **Date:** 2026-10-21
- **Host:** Windows 11 Pro 10.0.26200, x64, Ryzen 5 5500, 16 GB
- **Target:** FL Studio 2026 `26.1.7.5654`
- **Install root:** `C:\Program Files\Image-Line\FL Studio 2026`
- **Analysis authority:** shipped artifact, read-only local inspection and hashes
- **REA:** 6.7.0, Ghidra 12.1.4, Temurin JDK 21.0.12.1

## Hashes

| File | Size | SHA-256 |
| --- | ---: | --- |
| `FL64.exe` | 597,064 | `A1F3F53CB8EE1BB80FDFE1814876E268730F8B6BDBA95E3D4A8BCBA6600255E6` |
| `FLEngine_x64.dll` | 22,227,016 | `6D9C5AB09C1D5594012AC2C31CF9ED08E73F42164ED0964CCCAE10A95D9B8E02` |

The scaled launcher `FL64 (scaled).exe` is the same size and version metadata as
`FL64.exe`; treat it as a UI-scaling launcher variant until proven otherwise.

## Process and component inventory

Observed executables:

- `FL64.exe` — main application launcher.
- `Shared\ILPluginScanner64.exe` — plugin scanner process.
- `Shared\ILMinihostBridge64.exe` — plugin bridge/minihost process.
- `Shared\Python\python.exe` and `pythonw.exe` — embedded Python runtime.
- `System\Tools\Bridge\32bit` and `64bit` — bridge helpers.
- `System\Tools\BeatSlicer`, `Control Creator`, `Diagnostics`, `License`,
  `LilyPond` — bundled tools.

Observed native components:

- `FLEngine_x64.dll` — core engine.
- `speaker_x64.dll` — audio output layer.
- `AudioRestore.dll`, `iZAudioRestore.dll` — audio restoration.
- `FLMManaged.dll` — managed component; inspect separately if needed.
- `Shared\Reverb_x64.dll`, `ildsp_x64.dll`, `dsp_ippv2_x64.dll`,
  `elastique_x64.dll`, `DirectWaveCtEngine_x64.dll`, `beat_tracker.dll` — DSP and
  engine components.
- `Shared\LAMEenc_x64.dll`, `oggio_x64.dll`, `EasyFLAC_x64.dll`,
  `wavpackdll_x64.dll` — codec components.

## Plugin layout

- `Plugins\Fruity` — Image-Line's bundled plugin content.
- `Shared` — shared engine, scanner, bridge, codecs, and embedded runtime.
- `Artwork` — visual assets. **Do not copy.**
- `Data`, `Help`, `System` — content, documentation, and tools.

## Clean-room notes

This inventory is descriptive. It does not grant permission to copy any file,
asset, string, preset, or implementation. The following are out of scope:

- Extracting or porting proprietary DSP.
- Copying factory presets, samples, artwork, or UI assets.
- Recovering or bypassing activation/licensing.
- Shipping any FL Studio binary in this repository.

## Open questions for later phases

1. What is the exact plugin-host protocol between `FL64.exe`,
   `ILPluginScanner64.exe`, and `ILMinihostBridge64.exe`?
2. Which audio device APIs does `speaker_x64.dll` expose (WASAPI, ASIO, DirectSound)?
3. What is the on-disk structure of `.flp` and which chunks are required for a
   read-only importer?
4. Which UI behaviors are functional requirements (for example, pattern clips and
   step sequencing) versus incidental visual design?

These questions are investigated with REA in read-only mode and recorded as
behavioral specifications, not copied code.
