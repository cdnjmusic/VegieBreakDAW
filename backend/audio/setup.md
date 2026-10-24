# Audio Backend Setup

Target driver strategy:
- Primary: ASIO4ALL (universal ASIO wrapper) — recommend user install if not present
- Fallback: FL Studio ASIO (ILWASAPI2ASIO_x64.dll) — present on this system
- Windows Audio Session API (WASAPI) as lowest-latency safe fallback

No new audio driver created yet. Using existing ASIO infrastructure.
Future: evaluate if custom driver needed beyond ASIO4ALL coverage.
