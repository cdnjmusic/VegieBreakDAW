#requires -Version 5.1
<#
.SYNOPSIS
    Build VegieBreakDAW for the MSVC ABI without a Visual Studio installation.
.DESCRIPTION
    Uses LLVM's clang-cl/lld-link, the portable MSVC CRT downloaded by xwin, and
    the installed Windows SDK. This is the local workaround while Microsoft's
    Visual Studio installer catalog is inconsistent.

    Visual Studio Build Tools remain the recommended long-term toolchain for
    native plugin SDK work. This script proves the MSVC ABI path and keeps the
    project buildable in the meantime.
.EXAMPLE
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\build-msvc.ps1
#>
[CmdletBinding()]
param(
    [switch]$Release,
    [switch]$Test
)

$ErrorActionPreference = 'Stop'

$llvm = 'C:\Program Files\LLVM\bin'
$crtRoot = Join-Path $HOME 'AppData\Local\cargo-xwin\xwin\crt'
$sdkRoot = 'C:\Program Files (x86)\Windows Kits\10'
$sdkVersion = '10.0.26100.0'

foreach ($path in @(
    (Join-Path $llvm 'clang-cl.exe'),
    (Join-Path $llvm 'lld-link.exe'),
    (Join-Path $llvm 'llvm-lib.exe'),
    (Join-Path $crtRoot 'lib\x86_64\vcruntime.lib'),
    (Join-Path $sdkRoot "Lib\$sdkVersion\um\x64\kernel32.lib")
)) {
    if (-not (Test-Path $path)) {
        throw "MSVC build dependency missing: $path"
    }
}

$env:Path = "$llvm;C:\Tools\mingw64\bin;$env:Path"
$env:LIB = @(
    (Join-Path $crtRoot 'lib\x86_64'),
    (Join-Path $sdkRoot "Lib\$sdkVersion\um\x64"),
    (Join-Path $sdkRoot "Lib\$sdkVersion\ucrt\x64")
) -join ';'
$env:INCLUDE = @(
    (Join-Path $crtRoot 'include'),
    (Join-Path $sdkRoot "Include\$sdkVersion\ucrt"),
    (Join-Path $sdkRoot "Include\$sdkVersion\shared"),
    (Join-Path $sdkRoot "Include\$sdkVersion\um"),
    (Join-Path $sdkRoot "Include\$sdkVersion\winrt")
) -join ';'
$env:CC = 'clang-cl'
$env:CXX = 'clang-cl'
$env:AR = 'llvm-lib'
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = 'lld-link'

$cargo = Join-Path $HOME '.cargo\bin\cargo.exe'
if (-not (Test-Path $cargo)) { $cargo = 'cargo' }

$common = @('--workspace', '--target', 'x86_64-pc-windows-msvc')
if ($Release) { $common += '--release' }

Write-Host 'Building x86_64-pc-windows-msvc with LLVM + portable CRT + Windows SDK...'
& $cargo build @common
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if ($Test) {
    & $cargo test @common
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

Write-Host 'MSVC ABI build succeeded.' -ForegroundColor Green
