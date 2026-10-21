# Architecture

## Design constraints

1. **Audio correctness beats feature count.** A dropped buffer is a bug, not a
   cosmetic issue.
2. **The audio callback never allocates, locks, logs, blocks, or performs file I/O.**
3. **Plugin failures must not take down the application.** Plugins run out of process.
4. **Every project mutation is undoable and serializable.**
5. **The UI may drop frames; the audio engine may not.**
6. **No account, network, or license service is required to make music.**

## Workspace layout

```
crates/
  vb-core/     project model, tempo/time, validation, undo primitives
  vb-audio/    device abstraction, transport, graph, DSP scheduling
  vb-ui/       shell, theme, timeline/piano-roll renderers
  vb-host/     plugin discovery, loading, state, process isolation
  vb-stems/    source separation, model management, progress
  vb-io/       audio/MIDI decode, project persistence, export
  vb-app/      composition root, CLI, recovery, diagnostics
```

Dependencies point inward: `vb-core` depends on no other workspace crate.
`vb-app` is the only crate that composes all of them.

## Real-time audio path

```
device callback
  -> capture current transport snapshot (atomic/RCU)
  -> for each block:
       graph.process(block)
         -> instrument/sampler voices
         -> insert chain (per channel)
         -> sends/buses
         -> master chain
       graph.swap_output()
  -> write interleaved output
```

Rules:

- All buffers are allocated at graph-build time. The callback receives a
  preallocated `AudioBlock`.
- Parameter changes arrive through a bounded SPSC ring buffer (`rtrb` or an
  equivalent). The audio thread drains it at block boundaries and applies
  sample-accurate automation where required.
- No `Mutex`, `RwLock`, `std::fs`, `println!`, or `tracing` calls inside the
  callback. Diagnostics go through a lock-free counter/event ring.
- Denormals are flushed (`_MM_SET_FLUSH_ZERO_MODE` on x86) or handled with
  explicit DC-offset/denormal guards.
- The graph is topologically sorted once per structural change, not per block.
- Plugin delay compensation (PDC) is computed from reported plugin latency and
  applied with preallocated delay lines.
- Offline render uses the same graph with a deterministic block size and no
  device clock.

### Performance targets

| Metric | Target |
| --- | --- |
| Internal sample format | `f64` |
| Device buffer | 64 samples at 48 kHz (1.33 ms) |
| Round-trip latency (ASIO) | < 5 ms |
| Round-trip latency (WASAPI exclusive) | < 10 ms |
| Audio callback allocations | 0 |
| UI frame time | < 8.3 ms (120 Hz target) |
| Cold start, no plugins | < 2 s |
| Plugin scan | parallel, cached, UI never blocks |

ASIO is optional and requires the ASIO SDK, which is not redistributed. WASAPI
shared/exclusive is the default Windows backend. ASIO support is compiled only
when the user supplies the SDK.

## Plugin hosting

`vb-host` defines a format-neutral descriptor and host trait.

- **VST3**: COM-based, requires the VST3 SDK. The application is GPL-3.0-or-later
  to use the SDK's GPL option.
- **CLAP**: MIT-licensed C ABI; preferred for a clean, permissive path.
- **VST2**: legacy, optional, and only where the user has the right to use the
  VST2 ABI/SDK. Not a first-class target.
- **AAX**: Avid's SDK is proprietary and license-gated. The backend is an
  optional local feature; no Avid SDK code or binaries are distributed here.

### Process isolation

Plugins run in a separate `vb-plugin-host` process:

```
main process                         plugin process
  audio graph  <-- shared ring -->   plugin instance
  UI editor    <-- IPC commands -->  editor window (native)
  watchdog     <-- heartbeat ---->   health
```

The audio thread never calls plugin code directly. It writes blocks to a
shared-memory ring and reads the processed result. A crashed or hung plugin is
detected by a watchdog, bypassed, and reported; the project remains open.

This costs one buffer of latency per hosted plugin. The host compensates with
PDC and exposes the added latency in the mixer.

## Project model

The project is a versioned tree:

```
Project
  metadata, sample_rate, tempo, time_signature
  tracks[]        audio | instrument | automation
    clips[]       audio/MIDI/automation, start/length in beats
  patterns[]      notes, controller lanes
  mixer           channels, sends, master, plugin slots
  assets[]        content-addressed audio/MIDI references
```

Serialization is JSON for the first milestone, with a binary/MessagePack path
later for large projects. Asset paths are relative to the project file where
possible. The schema has an explicit `schema_version` and a migration table.

Undo/redo is command-based. Every user action produces a reversible command;
the model is never mutated from the audio thread.

## UI

The UI is GPU-rendered. The target stack is `egui` for controls plus a custom
`wgpu` renderer for the timeline, piano roll, and waveform views. The UI reads a
snapshot of the model and sends commands; it never mutates audio state directly.

Rendering rules:

- Virtualize clips and notes outside the viewport.
- Cache waveform min/max mipmaps; do not rescan audio per frame.
- Keep plugin editors in their own native windows.
- Run plugin scans, stem separation, and offline render on worker threads with
  progress and cancellation.

## Stem separation

The stem pipeline is a worker job:

```
input audio -> decode/resample to model rate -> chunk -> ONNX session
            -> overlap-add -> per-stem buffers -> project assets
```

The first model is an ONNX export of **HTDemucs** (MIT-licensed model and code).
The model is shipped as an optional asset pack or downloaded once with a SHA-256
manifest. Inference uses ONNX Runtime via the `ort` crate. The job reports
progress, supports cancellation, and never blocks the audio callback.

Output stems are sample-aligned with the input. The 4-stem set is vocals, drums,
bass, and other. A 6-stem model can be added later behind the same interface.

## File formats

- **Project**: `.vbproj` JSON (versioned).
- **Audio import**: WAV, FLAC, AIFF, OGG Vorbis (via `symphonia`).
- **Audio export**: WAV (float/24-bit/16-bit), FLAC.
- **MIDI**: SMF import/export (via `midly`).
- **FLP**: read-only import is a research item, not a compatibility promise.
  It will be implemented from clean-room format notes if it can be done without
  copying protected code.
- **Plugin state**: opaque chunks stored per plugin instance.

## Error handling and recovery

- `thiserror` for library errors; typed error codes in the UI.
- Autosave every 60 seconds and before destructive operations.
- Crash recovery journal in the project directory.
- Plugin crashes are contained and surfaced as a bypassed slot.
- Audio device loss triggers a controlled restart and a clear UI state.

## Security and privacy

- No network access by default.
- No telemetry, analytics, or account identifiers.
- Model downloads, if enabled, are explicit, checksum-verified, and cancellable.
- Project files are untrusted input: validate sizes, chunk lengths, and paths
  before decoding.
