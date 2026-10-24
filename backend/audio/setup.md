# Audio Backend Setup

Driver status (verified 2026-10-24):
- ASIO4ALL v2 installed: C:\Program Files (x86)\ASIO4ALL v2
  - a4apanel.exe (control panel, 40 KB)
  - asio4all.dll (32-bit, 370 KB)
  - asio4all64.dll (64-bit, 406 KB)
  - uninstall.exe
- FL Studio ASIO driver present: C:\Program Files\Image-Line\FL Studio ASIO\ILWASAPI2ASIO_x64.dll
- ASIO4ALL installer in Downloads: ASIO4ALL_2_22.exe (557 KB) — matches installed version

Target driver strategy for VegieBreakDAW:
- Primary: ASIO4ALL 64-bit (asio4all64.dll) — active and installed
- Fallback: FL Studio ASIO (ILWASAPI2ASIO_x64.dll) — verified working
- Lowest fallback: WASAPI

No custom audio driver created. Backend integrates with installed ASIO4ALL universal wrapper directly; references the 64-bit DLL path above.
