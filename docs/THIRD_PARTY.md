# Third-party notices

VegieBreakDAW is GPL-3.0-or-later. The following components are planned or used.
This file must be updated whenever a dependency, model, or SDK is added.

## Rust crates

| Crate | Purpose | License |
| --- | --- | --- |
| `serde` / `serde_json` | serialization | MIT / Apache-2.0 |
| `thiserror` | error types | MIT / Apache-2.0 |
| `tracing` | diagnostics | MIT |
| `cpal` (planned) | Windows audio devices | Apache-2.0 |
| `symphonia` (planned) | audio decoding | MPL-2.0 |
| `midly` / `midir` (planned) | MIDI files and I/O | MIT |
| `egui` / `wgpu` (planned) | UI and GPU rendering | MIT / Apache-2.0 |
| `ort` (planned) | ONNX Runtime bindings | MIT / Apache-2.0 |
| `rtrb` (planned) | lock-free ring buffer | MIT / Apache-2.0 |

## SDKs and formats

| Component | Status | License / terms |
| --- | --- | --- |
| CLAP | planned first-class host | MIT |
| VST3 SDK | planned host | GPL-3.0 or Steinberg proprietary license |
| VST2 SDK/ABI | legacy optional | not redistributed; user must hold rights |
| AAX SDK | optional, not distributed | Avid proprietary license |
| ASIO SDK | optional, not distributed | Steinberg proprietary license |

The application is GPL-3.0-or-later so the VST3 SDK's GPL option is available.
We do not distribute the AAX or ASIO SDKs. AAX support is a local build option
for users who have accepted Avid's terms.

## Models

### HTDemucs (stem separation)

- Code and model: Facebook Research / Meta AI.
- License: MIT.
- Source: <https://github.com/facebookresearch/demucs>
- ONNX export option: <https://huggingface.co/timcsy/demucs-web-onnx>
- The model is an optional asset. Its license text is stored alongside the model
  manifest when downloaded or bundled.

## Fonts and visual assets

- UI fonts and icons must be original or licensed for redistribution.
- No FL Studio artwork, icons, themes, or factory images may be committed.

## REA

- `rea-agents` is MIT-licensed and used only as a development/analysis tool. It is
  not linked into or shipped with VegieBreakDAW.
