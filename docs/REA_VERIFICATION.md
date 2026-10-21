# REA verification — 2026-10-21

## Installed identity

- `rea-agents` **6.7.0**
- MCP server **rea 6.7.0**, started through `cmd.exe /d /c npx -y rea-agents@6.7.0 mcp`
- MCP SDK **2.3.1**
- Skill **reverse-engineer-anything** version 42, 207 tools
- Client registration: `C:\Users\Potaku\.dsh\cordis.patch.yml`
- Provider: **Ghidra 12.1.4**
- JDK: **Temurin 21.0.12.1**, 64-bit
- Ghidra path: `C:\Tools\ghidra_12.1.4_PUBLIC`

## Doctor

`npx -y rea-agents@6.7.0 doctor --client dsh --skill --json`:

- `scope_checks`: node, registration, skill identity — all healthy.
- `identity.skill.state`: `aligned`.
- `identity.registrations[0].state`: `aligned`.
- `availableProviders`: `ghidra`.

`npx -y rea-agents@6.7.0 doctor --provider ghidra --json`:

- `healthy: true`
- `environment_healthy: true`
- `ghidra-version`: `12.1.4`
- `ghidra-native_decompiler`: `pe x86_64`, matches `win32/x64`
- `ghidra-java`: Temurin 21, 64-bit, full JDK
- `ghidra-native_authority`: Windows Job Object and private DACL verified

## Live MCP verification

`binary_session` (expected package `6.7.0`):

- `server_identity.package.version`: `6.7.0`
- `server_identity.skill.expected_version`: `42`
- `analysis_provider_candidates[ghidra].availability.status`: `available`
- Ghidra diagnostics report the correct install, headless script, native
  decompiler, and Java paths.

`open_binary` on the installed FL Studio launcher:

```text
C:\Program Files\Image-Line\FL Studio 2026\FL64.exe
sha256 a1f3f53cb8ee1bb80fdfe1814876e268730f8b6bdba95e3d4a8bcba6600255e6
format pe, architecture x86_64, kind executable
```

The target was accepted read-only. A full `binary_overview` request on the
launcher exceeded the initial client request window, so the session was closed
and a smaller smoke PE was used to verify end-to-end Ghidra analysis.

## End-to-end Ghidra smoke test

A 61 KB MinGW PE (`C:\Tools\rea-smoke.exe`, SHA-256
`8f23563ef35531f53a78eefd3bf8fb8e26624e92f412fbb39a05a335484ea211`)
was opened with `provider_id: "ghidra"`.

`binary_overview` returned:

- image base `0x140000000`
- 19 segments, including `.text` at `0x140001000`
- **145 procedures**
- **267 strings**

This confirms that the MCP server, Ghidra launcher, JDK, native decompiler, and
Evidence path all work end to end on Windows x64.

## Registration environment

The DSH registration now carries:

```yaml
env:
  GHIDRA_INSTALL_DIR: C:\Tools\ghidra_12.1.4_PUBLIC
  JAVA_HOME: C:\Program Files\Eclipse Adoptium\jdk-21.0.12.101-hotspot
```

This keeps the provider configuration stable across shells and Harness
restarts. The setup backup is `C:\Users\Potaku\.dsh\cordis.patch.yml.rea.backup`.

## Scope reminder

The launcher was opened read-only. No FL Studio code, assets, presets, or
license data were extracted or copied. The full FL Studio analysis will follow
the clean-room policy in [`CLEANROOM.md`](CLEANROOM.md).
