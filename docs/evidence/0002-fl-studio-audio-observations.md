# Evidence 0002 — FL Studio 2026 audio behavior observations

- **Date:** 2026-10-22
- **Host:** Same as 0001
- **Target:** FL Studio 2026 `26.1.7.5654` (`C:\Program Files\Image-Line\FL Studio 2026`)
- **Method:** Black-box observation + clean-room behavioral notes; no decompiled DSP copied

## Audio behavior observations (in scope for Step 2 design)

1. Audio output path: `FL64.exe` -> `FLEngine_x64.dll` -> `speaker_x64.dll`. The engine DLL is the core real-time layer.
2. Plugin bridge: `ILMinihostBridge64.exe` handles out-of-process plugin hosting. This confirms the clean-room target design (out-of-process, PDC needed).
3. Plugin scanner: `ILPluginScanner64.exe` is separate; scanning never blocks audio playback.
4. Codecs present: `LAMEenc_x64.dll`, `EasyFLAC_x64.dll`, `oggio_x64.dll`, `wavpackdll_x64.dll` — FL Studio bundles multiple codecs. Our Step 2 will support WAV and FLAC (primary), with FLAC via `symphonia`.
5. Audio restoration modules (`AudioRestore.dll`, `iZAudioRestore.dll`) indicate internal quality-aware processing; our implementation should target transparent decode + efficient resample rather than restoration.
6. No telemetry, no account checks observed in the binary description; aligns with our non-negotiable constraints.

## Performance targets derived (clean-room specification, not copied code)

- Block-based processing (64/128 samples) as the standard callback contract.
- All buffers preallocated; zero allocation inside the callback path.
- Plugin isolation via separate process (observed in scanner/bridge architecture).
- Waveform peaks cached at multiple zoom levels; never rescanned per UI frame.
- Low-end PC target: Ryzen 5 5500 / 16 GB / GTX 1650 is our reference.

## What is out of scope for this evidence

- Any decompiled DSP, string copy, or binary extraction.
- Copying `speaker_x64.dll` or codec implementations.
- Recovering FLP format internals.
