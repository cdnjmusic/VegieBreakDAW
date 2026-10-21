# Security policy

VegieBreakDAW handles untrusted project files, audio, MIDI, and third-party
plugins. We take memory safety and process isolation seriously.

## Supported versions

The project is pre-1.0. Security fixes land on `main` and the latest release.

## Reporting

Please report vulnerabilities privately to the maintainers before public
disclosure. Include:

- Affected version or commit.
- A minimal reproduction.
- Impact and any known mitigations.

Do not include proprietary FL Studio binaries, license data, or user audio in a
report.

## Design commitments

- No account, telemetry, or network access by default.
- Plugin hosting is process-isolated once the host lands.
- Project and media parsers validate sizes, offsets, and paths before use.
- Model downloads are explicit, checksum-verified, and cancellable.
- No license-bypass or unlock functionality will be accepted.
