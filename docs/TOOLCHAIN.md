# Windows toolchain notes

## Current local setup

| Tool | Version | Location |
| --- | --- | --- |
| Rust | 1.99.1, GNU host | `%USERPROFILE%\.cargo` |
| MinGW-w64 UCRT | GCC 16.2.0 | `C:\Tools\mingw64` |
| LLVM | 23.1.4 | `C:\Program Files\LLVM` |
| Windows SDK | 10.0.26100.0 | `C:\Program Files (x86)\Windows Kits\10` |
| MSVC CRT | 14.44.35207 | `%LOCALAPPDATA%\cargo-xwin\xwin\crt` |
| CMake | 4.4.4 | `C:\Program Files\CMake` |
| Ninja | 1.13.2 | winget package directory |
| JDK | Temurin 21.0.12.1 | `C:\Program Files\Eclipse Adoptium` |
| Ghidra | 12.1.4 | `C:\Tools\ghidra_12.1.4_PUBLIC` |

`rustup default stable-x86_64-pc-windows-gnu` is selected locally so a plain
`cargo build` works. GitHub CI uses `stable-x86_64-pc-windows-msvc`.

## Visual Studio installer issue

The Visual Studio Build Tools bootstrapper fails with:

```text
0x80131509: Unable to cache catalog using channel uri
https://aka.ms/vs/17/release/channel
```

The channel manifest advertises a `VisualStudio.vsman` catalog of about
30,443,686 bytes, but the CDN currently serves 17,954,770 bytes with a different
SHA-256. The same mismatch appears on the 17.12 and 17.10 LTSC channels and on
the VS 2019 channel, so the local installer cannot validate the catalog.

Do not disable signature/hash validation to work around this.

## MSVC ABI workaround

`scripts/build-msvc.ps1` builds the workspace for
`x86_64-pc-windows-msvc` without Visual Studio:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\build-msvc.ps1 -Test
```

It sets:

- `CC` / `CXX` = `clang-cl`
- `AR` = `llvm-lib`
- `CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER` = `lld-link`
- `LIB` = portable MSVC CRT + Windows SDK `um\x64` + `ucrt\x64`
- `INCLUDE` = portable MSVC CRT headers + Windows SDK headers

This is enough for the current pure-Rust workspace and for linking the MSVC Rust
target. Native C++ SDK work may still need the full Visual Studio Build Tools
installation; retry it when Microsoft's catalog is consistent.

## Verification

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\verify-setup.ps1
powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\build-msvc.ps1 -Test
cargo test --workspace
```
