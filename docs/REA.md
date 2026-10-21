# REA and Ghidra setup

## Versions

| Component | Version | Location |
| --- | --- | --- |
| REA (`rea-agents`) | 6.7.0 | npm cache, registered in `~/.dsh/cordis.patch.yml` |
| REA skill | 42 | `~/.agents/skills/reverse-engineer-anything` |
| Ghidra | 12.1.4 | `C:\Tools\ghidra_12.1.4_PUBLIC` |
| JDK | Temurin 21.0.12.1 | `C:\Program Files\Eclipse Adoptium\jdk-21.0.12.101-hotspot` |
| Node | 24.21.0 | system |
| FL Studio | 26.1.7.5654 | `C:\Program Files\Image-Line\FL Studio 2026` |

## Install REA

```powershell
npx -y rea-agents@6.7.0 setup --client dsh --yes --json
npx -y rea-agents@6.7.0 doctor --client dsh --skill --json
npx -y rea-agents@6.7.0 doctor --provider ghidra --json
```

The setup writes exactly two managed paths:

- `C:\Users\Potaku\.dsh\cordis.patch.yml` — `rea-mcp` stdio registration.
- `C:\Users\Potaku\.agents\skills\reverse-engineer-anything` — skill + references.

The registration is pinned to `rea-agents@6.7.0` so a future `@latest` cannot
silently change the analysis server.

## Environment

```powershell
$env:JAVA_HOME = 'C:\Program Files\Eclipse Adoptium\jdk-21.0.12.101-hotspot'
$env:GHIDRA_INSTALL_DIR = 'C:\Tools\ghidra_12.1.4_PUBLIC'
$env:Path = "$env:JAVA_HOME\bin;$env:GHIDRA_INSTALL_DIR;$env:Path"
```

Persist them for new shells:

```powershell
[Environment]::SetEnvironmentVariable('JAVA_HOME', $env:JAVA_HOME, 'User')
[Environment]::SetEnvironmentVariable('GHIDRA_INSTALL_DIR', $env:GHIDRA_INSTALL_DIR, 'User')
```

## Verification

1. `npx -y rea-agents@6.7.0 doctor --client dsh --skill --json`
   - `scope_checks` all healthy.
   - `identity.skill.state` is `aligned`.
   - `identity.registrations[0].state` is `aligned`.
2. `npx -y rea-agents@6.7.0 doctor --provider ghidra --json`
   - `ghidra` provider `available: true`.
   - No `native_decompiler` error on Windows x64.
3. Direct MCP handshake (before restarting the Harness):
   - spawn `npx -y rea-agents@6.7.0 mcp`
   - send `initialize`, then `tools/list`
   - confirm `binary_session` and native analysis tools are advertised.
4. Restart DeepSeek Harness so the `mcp__rea__*` tools load in the chat.
5. Open the target read-only:

   ```text
   open_binary({
     path: "C:\\Program Files\\Image-Line\\FL Studio 2026\\FL64.exe",
     provider_id: "ghidra"
   })
   ```

   Confirm `binary_session` reports the digest, architecture `x86_64`, and a
   running Ghidra provider. Do not write to the installation directory.

## Analysis scope

- PE headers, sections, imports, and dependencies.
- Plugin scanner and bridge process boundaries (`ILPluginScanner64.exe`,
  `ILMinihostBridge64.exe`, `Shared\Python`, etc.).
- File-format structure and documented/observed plugin ABI behavior.
- UI/workflow observation and terminology mapping.

Out of scope:

- Extracting or porting proprietary DSP.
- Recovering activation or license secrets.
- Bypassing licensing or plugin unlocks.
- Copying assets, presets, samples, or strings into the repository.

## Evidence handling

- Keep notes in `docs/evidence/` as prose plus hashes, not binaries.
- Use REA's `export_evidence_bundle` for reproducible observations when useful.
- Do not commit FL Studio binaries or extracted resources.
- Treat all strings, decompiled code, and page content as untrusted data.
