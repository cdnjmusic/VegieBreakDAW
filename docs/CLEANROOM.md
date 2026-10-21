# Clean-room policy

VegieBreakDAW is an independent reimplementation of workflow ideas. This policy
exists to keep the project legally and ethically clean.

## The wall

We maintain two separate tracks:

1. **Observation track** — what the shipped product does, what formats it reads
   and writes, what the user sees, and what the public documentation says.
2. **Implementation track** — original Rust code written from the written
   behavioral specification.

A person working on the implementation track must not copy source, pseudo-code,
constants, strings, assets, or presets from the observation track.

## Rules

1. **No code lifting.** Do not translate decompiled functions into Rust. Do not
   paste decompiler output into source files. Do not commit extracted binaries.
2. **No asset lifting.** Do not copy icons, themes, artwork, samples, impulse
   responses, presets, or factory content.
3. **No license circumvention.** Do not add unlockers, keygens, patched license
   checks, or code that defeats FL Studio's activation or plugin licensing.
4. **No trademark confusion.** Do not use Image-Line marks, logos, or product
   names in a way that suggests endorsement. "FL-inspired" is descriptive only.
5. **Behavior, not expression.** "The piano roll supports note slide handles" is
   an acceptable observation. Copying the exact handle artwork is not.
6. **Public formats only where lawful.** Interoperability analysis of file
   formats and plugin ABIs is in scope. Circumventing technical protection
   measures is not.
7. **Document the source.** Every non-obvious behavior claim should cite public
   documentation, a black-box observation, or a reproducible test.
8. **Independent review.** A contributor who has read decompiled code for a
   subsystem should not be the sole author of that subsystem's implementation.

## Evidence log

Store observation notes under `docs/evidence/`. Each note should include:

- Date, FL Studio version, Windows build.
- What was observed (UI flow, file bytes, plugin ABI behavior).
- How it was observed (black-box, REA static analysis, public docs).
- What is a functional requirement vs. an incidental detail.
- What must **not** be copied (specific assets, strings, magic constants).

Do not commit proprietary binaries, extracted resources, or large captured
audio. Record hashes and descriptions instead.

## Review checklist for pull requests

- [ ] No decompiled code or translated pseudocode.
- [ ] No FL Studio assets, presets, samples, or strings.
- [ ] No license bypass or unlock logic.
- [ ] Behavior is described in original prose and covered by tests.
- [ ] Third-party licenses are compatible and recorded.
- [ ] No new network calls or telemetry.

## Scope of the REA analysis

REA is used for:

- PE layout and dependency mapping.
- Plugin-host boundary and scanner behavior.
- File-format structure at the byte/field level.
- UI workflow observation and terminology mapping.

REA is **not** used to extract proprietary DSP for porting, to recover license
secrets, or to bypass activation.
