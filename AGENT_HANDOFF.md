# AGENT_HANDOFF.md — Vega Break Studio

Mandatory instruction for any agent:
"Before starting work, read AGENT_HANDOFF.md and TASKS.json, inspect the actual repository state, and verify the relevant task against the code. Before finishing, update TASKS.json and update AGENT_HANDOFF.md only if its persistent project context has changed."

---

## Project Overview

Purpose: Vega Break Studio — an open-source DAW inspired by FL Studio and Logic Pro X. Focus: a clean, modern control bar (Logic-style) with neon gradient styling, real asset references, and interactive transport controls. Long-term vision includes video dubbing, cinematic sound placement, live performance, and scoring tabs.

Scope (current): Rust desktop application (`vegie-break-studio`) with `egui` GUI framework. No audio engine replacement; existing playback infrastructure preserved.

---

## Technology Stack

- Language: Rust
- GUI framework: `egui` (v0.31) via `eframe`
- Window backend: `winit` (via `eframe` / `egui_winit`)
- Build: `cargo` (`release` profile)
- Project root: `C:\Users\Potaku\Documents\REA_FL`
- Main binary: `vegie-break-studio/`

No additional audio engine or video codec libraries are integrated yet; `backend/` contains reference/spec files only.

---

## Architecture

Root structure:
- `.kilo/` — Agent/config files (`agent/doctor.md`, `command/doctor-check.md`)
- `branding/` — Logos (`app_icon_logo.png`, `no_bg_logo.png`, `whimsical_cartoon_logo.png`), icons (`play.svg`, `record.svg`, `loop.svg`, `snap.svg`, `tempo.svg`), colors (`scheme.md`), fonts (empty, ready)
- `references/` — DAW reference materials (`fl-studio/`, `logic-pro/`, etc.)
- `backend/` — Backend specs (`audio/setup.md`, `video/setup.md`, `tabs/overview.md`)
- `docs/` — `architecture_notes.md` (FL Studio clean-room observations)
- `vegie-break-studio/` — Main Rust application
  - `src/main.rs` — Entry point, workspace layout, `App` implementation
  - `src/control_bar.rs` — Dedicated toolbar component (transport, tempo, snap, branding)
  - `src/toolbar.md` — Toolbar design reference
  - `Cargo.toml` / `Cargo.lock` — Dependencies (`eframe`)
- `rea/` — Cloned `morluto/rea` repository; excluded from version control (`.gitignore` excludes it). Do NOT modify.
- `vegie-break-studio.exe` — Compiled binary copied to root for quick testing.
- `.gitignore` — Excludes `rea/` and `vegie-break-studio/target/`

Data flow: `main()` creates `EventLoop` and `App`. `App::update()` calls `control_bar::ControlBar::show()` inside a top panel, then renders the workspace in a central panel. No persistent state storage; no database; no file I/O beyond asset references.

---

## Current Implementation

Implemented (verified by code inspection and build):
- `egui` / `eframe` window (1280x720, title "Vegie Break Studio")
- `control_bar.rs`: top toolbar with Play, Record, Loop, Tempo, Snap buttons; interactive hover (`hot_pink`) / active (`gold`) states; clean divider line; branding label
- `branding/colors/scheme.md`: neon gradient palette (cyan `#00f0ff`, purple `#a020f0`, neon green `#00ff88`, gold `#ffcc00`, hot pink `#ff66cc`, slate `#0f0f16`)
- `branding/icons/*.svg`: 5 transport icons (play, record, loop, snap, tempo)
- `references/`: Logic Pro and FL Studio references
- `.gitignore`: excludes `rea/` and `target/`
- `AGENT_HANDOFF.md` and `TASKS.json` (this file and the JSON file) maintained as persistent tracking

Not fully implemented / needs verification:
- Real SVG icons are not rendered as image widgets in `egui`; buttons use text labels referencing the SVG assets (`branding/icons/*.svg`)
- The toolbar does not yet load the PNG logos (`branding/logos/*.png`)
- Transport controls (Play/Record/Loop/Tempo/Snap) are not connected to actual audio/transport state; buttons are visual-only
- Video backend (`backend/video/`) and scoring/live tabs (`backend/tabs/`) are spec files only
- No tests or automated verification beyond `cargo build --release`

---

## Design Principles

UI direction: Logic Pro X control bar reference (compact desktop DAW proportions, transport group, LCD-style position/tempo display, metronome, cycle). Modern neon gradient styling (not pastel, not generic). No footer bar.

References:
- `references/logic-pro/logic-pro-mac-user-guide.pdf` (primary visual reference for toolbar layout)
- `references/fl-studio/FL_link.txt`
- `branding/colors/scheme.md` (authoritative color palette)

Constraints:
- Keep `rea/` untouched; all development stays outside it (`REA_FL` folder)
- No redesign of unrelated modules; build/test must pass before reporting done
- Use `TASKS.json` as the authoritative status source; update `AGENT_HANDOFF.md` only when persistent context changes

---

## Important Decisions

- GUI framework: `egui` / `eframe` (chosen for lightweight native rendering, easy custom styling, no HTML/web dependency)
- Build profile: `release` used for `.exe` output (`vegie-break-studio.exe`)
- Brand assets stored in `branding/`, separated from application code
- Reference materials stored in `references/`, not embedded in `docs/`
- No footer bar (user requirement); workspace uses central panel only
- ASIO4ALL (`C:\Program Files (x86)\ASIO4ALL v2\asio4all64.dll`) installed but not yet integrated into audio engine; FL Studio ASIO (`ILWASAPI2ASIO_x64.dll`) confirmed present as fallback

---

## Known Issues

- Toolbar buttons show text labels (`"Play"`, `"Record"`, etc.) instead of rendered SVG icons from `branding/icons/`. `egui` requires image loading setup for SVG; this needs resolution or a PNG conversion step.
- The gradient divider line (`line_segment`) is a thin cyan line, not a full gradient strip; acceptable per latest design direction but should be validated against the Logic Pro reference.
- No automated test suite; verification relies on manual `cargo build --release` and visual inspection of `.exe` output.
- `.kilo/command/doctor-check.md` and `.kilo/agent/doctor.md` configured for health-check agent but not actively tested in CI.
- `rea/` is excluded by `.gitignore`; ensure nothing from it leaks into commits.

---

## Build & Verification

Build command:
```
cd C:\Users\Potaku\Documents\REA_FL\vegie-break-studio
cargo build --release
```

Verification requirements (per user instructions):
- Build succeeds (`Finished release profile`)
- `.exe` rebuilt (`vegie-break-studio.exe` in root updated after build)
- `TASKS.json` updated with accurate status, files, verification
- Visual inspection performed if user requires it (run `.exe` and compare to Logic Pro reference image)

No automated tests exist. Do not claim visual verification unless `.exe` was actually launched and inspected.

---

## Task Tracking Reference

`TASKS.json` is the authoritative source. See it for current priorities, blockers, and verification status. Key active/in-progress tasks:

- `UI-001`: Logic-inspired DAW control bar (transport, LCD, icons). Status should reflect actual build/verification state. Reference image: `references/logic-pro/logic-pro-mac-user-guide.pdf` and the referenced FL Studio interface images.
- `UI-002`: Branding/logos integration (`branding/logos/` PNG files not yet loaded into toolbar).
- `AUDIO-001`: ASIO4ALL integration (driver present at `C:\Program Files (x86)\ASIO4ALL v2` but not yet linked to audio engine).
- `BUILD-001`: `.gitignore` maintenance (exclude `rea/`, `target/`).

---

## Instructions for Future Agents

Before working:
1. Read this file (`AGENT_HANDOFF.md`).
2. Read `TASKS.json`.
3. Inspect `git status`, `git log --oneline -3`, and the actual repository contents.
4. Verify the selected task's files and acceptance criteria against the code.
5. Identify dependencies (e.g., toolbar changes depend on `branding/` assets; audio changes depend on `GHIDRA_INSTALL_DIR` / `JAVA_HOME` environment).

When working:
- Modify only files in `REA_FL` (not `rea/`).
- Do not replace unrelated modules (`main.rs` structure, `Cargo.toml` framework choice should stay unless explicitly required).
- After implementation: build (`cargo build --release`), update `.exe` if required, update `TASKS.json`, update `AGENT_HANDOFF.md` only if persistent context changed.

After finishing:
- Report: what files changed, build/test results, visual verification (if performed), remaining blockers.
- Update `TASKS.json` before final message.
- Leave precise notes for next agent in `TASKS.json` (notes/dependencies/blockers) and this file if architecture changed.
- Never claim a build/test/visual inspection succeeded unless it actually occurred.
