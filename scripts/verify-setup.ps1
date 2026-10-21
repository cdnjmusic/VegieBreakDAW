#requires -Version 5.1
<#
.SYNOPSIS
    Verify the VegieBreakDAW development and analysis toolchain.
.DESCRIPTION
    Read-only checks for Git, Rust, a C toolchain, CMake, Ninja, JDK, Ghidra,
    and the REA MCP registration. Does not install or modify anything.
.EXAMPLE
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts\verify-setup.ps1
#>
[CmdletBinding()]
param(
    [switch]$SkipRea
)

$ErrorActionPreference = 'Stop'

# Winget and rustup update the user PATH, but an already-running shell does not
# see it. Rebuild the effective PATH for this check.
$machinePath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$env:Path = "$machinePath;$userPath"

# User-level variables set after this shell started are not inherited. Load
# them explicitly so the checks and REA child process see the same values.
foreach ($name in 'JAVA_HOME', 'GHIDRA_INSTALL_DIR') {
    if (-not (Test-Path "Env:$name")) {
        $value = [Environment]::GetEnvironmentVariable($name, 'User')
        if ($value) { Set-Item -Path "Env:$name" -Value $value }
    }
}

$results = [System.Collections.Generic.List[object]]::new()

function Add-Result {
    param(
        [string]$Name,
        [bool]$Ok,
        [string]$Detail
    )
    $results.Add([pscustomobject]@{
        Check  = $Name
        Status = if ($Ok) { 'PASS' } else { 'FAIL' }
        Detail = $Detail
    })
}

function Find-Tool {
    param(
        [string]$Name,
        [string[]]$Fallbacks = @()
    )
    $command = Get-Command $Name -ErrorAction SilentlyContinue
    if ($command) { return $command.Source }
    foreach ($candidate in $Fallbacks) {
        if (Test-Path $candidate) { return $candidate }
    }
    return $null
}

$git = Find-Tool 'git' @('C:\Program Files\Git\cmd\git.exe')
Add-Result 'git' ($null -ne $git) $(if ($git) { & $git --version } else { 'not found' })

$cargo = Find-Tool 'cargo' @("$HOME\.cargo\bin\cargo.exe")
$rustc = Find-Tool 'rustc' @("$HOME\.cargo\bin\rustc.exe")
Add-Result 'rustc' ($null -ne $rustc) $(if ($rustc) { & $rustc --version } else { 'not found' })
Add-Result 'cargo' ($null -ne $cargo) $(if ($cargo) { & $cargo --version } else { 'not found' })

$gcc = Find-Tool 'gcc' @('C:\Tools\mingw64\bin\gcc.exe')
Add-Result 'C toolchain' ($null -ne $gcc) $(if ($gcc) { (& $gcc --version | Select-Object -First 1) } else { 'gcc not found' })

$cmake = Find-Tool 'cmake' @('C:\Program Files\CMake\bin\cmake.exe')
Add-Result 'cmake' ($null -ne $cmake) $(if ($cmake) { (& $cmake --version | Select-Object -First 1) } else { 'not found' })

$ninja = Find-Tool 'ninja'
Add-Result 'ninja' ($null -ne $ninja) $(if ($ninja) { & $ninja --version } else { 'not found' })

$java = Find-Tool 'java'
$javaVersion = 'not found'
if ($java) {
    $previousEap = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    $javaVersion = (& $java -version 2>&1 | Select-Object -First 1)
    $ErrorActionPreference = $previousEap
}
Add-Result 'java' ($null -ne $java) $javaVersion

$ghidra = $env:GHIDRA_INSTALL_DIR
$ghidraOk = $ghidra -and (Test-Path (Join-Path $ghidra 'support\analyzeHeadless.bat'))
Add-Result 'Ghidra' ([bool]$ghidraOk) $(if ($ghidraOk) { $ghidra } else { 'GHIDRA_INSTALL_DIR missing or invalid' })

$jdk = $env:JAVA_HOME
$jdkOk = $jdk -and (Test-Path (Join-Path $jdk 'bin\java.exe'))
Add-Result 'JAVA_HOME' ([bool]$jdkOk) $(if ($jdkOk) { $jdk } else { 'JAVA_HOME missing or invalid' })

if (-not $SkipRea) {
    try {
        $doctorJson = npx -y rea-agents@6.7.0 doctor --client dsh --skill --json 2>$null
        $doctor = $doctorJson | ConvertFrom-Json
        Add-Result 'REA MCP' ([bool]$doctor.healthy) $(if ($doctor.healthy) { 'healthy' } else { 'doctor reported unhealthy' })
        $ghidraProvider = $doctor.providerInspections | Where-Object { $_.id -eq 'ghidra' } | Select-Object -First 1
        Add-Result 'REA Ghidra provider' ([bool]$ghidraProvider.available) $(if ($ghidraProvider.available) { $ghidraProvider.providerVersion } else { 'unavailable' })
    } catch {
        Add-Result 'REA MCP' $false $_.Exception.Message
    }
}

$results | Format-Table -AutoSize

$failed = @($results | Where-Object { $_.Status -eq 'FAIL' })
if ($failed.Count -gt 0) {
    Write-Error "$($failed.Count) setup check(s) failed."
    exit 1
}

Write-Host 'All setup checks passed.' -ForegroundColor Green
exit 0
