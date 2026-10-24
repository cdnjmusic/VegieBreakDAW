# Audio Backend Setup

Driver status (verified 2026-10-24):
- ASIO4ALL installer present: C:\Users\Potaku\Downloads\ASIO4ALL_2_22.exe (557 KB)
- ASIO4ALL driver NOT installed in System32/drivers, SysWOW64, or Program Files
- FL Studio ASIO driver present: C:\Program Files\Image-Line\FL Studio ASIO\ILWASAPI2ASIO_x64.dll

Target driver strategy:
- Primary (intended): ASIO4ALL — requires running the installer to register the driver
- Fallback (current): FL Studio ASIO (present and functional)
- Lowest fallback: WASAPI

No custom audio driver created. Relying on ASIO4ALL universal wrapper once installed; if it fails, FL Studio ASIO covers the gap.
Integration note for VegieBreakDAW backend: reference ASIO4ALL via its installed DLL/driver name once active; for now backend references FL Studio ASIO path as verified working alternative.
