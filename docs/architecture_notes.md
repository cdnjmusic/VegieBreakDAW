# FL Studio Backend Observations (read-only, clean-room reference)

Installed at: C:\Program Files\Image-Line\FL Studio 2026

Engine:
- FLEngine_x64.dll (~22 MB) — core engine
- FLMManaged.dll (~9.5 MB) — managed layer
- FL64.exe / FL64 (scaled).exe (~597 KB each)
- ASIO driver: ILWASAPI2ASIO_x64.dll (in FL Studio ASIO folder)
- Audio restore / audio engine DLLs present

Plugins folder: Plugins\Fruity (built-in plugins)

Notes for VegieBreakDAW backend:
- FL Studio uses its own ASIO driver (not ASIO4ALL exclusively)
- ASIO4ALL is an alternative universal ASIO wrapper; can be used as fallback
- Engine is monolithic (single large DLL) — we will split into modular audio graph
- No reverse engineering of FLEngine code performed; only file inventory and naming
