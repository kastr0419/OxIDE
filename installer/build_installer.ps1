<#
PowerShell helper to build OxIDE Windows installer.
This script will:
  - Build the release binary (cargo build --release)
  - Download rustup-init.exe to installer/tools\rustup-init.exe
  - Attempt to download avrdude.exe to installer/tools\avrdude.exe (may fail; see notes)
  - Run Inno Setup Compiler (ISCC.exe) to build the installer

NOTE: This script does not execute automatically as part of the repository — run it manually.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$repoRoot = Resolve-Path (Join-Path $scriptDir '..')
$toolsDir = Join-Path $scriptDir 'tools'
$outputDir = Join-Path $scriptDir 'output'

Write-Host "Script dir: $scriptDir"
Write-Host "Tools dir: $toolsDir"

# 1) Build release binary
Write-Host "Building release binary..."
Push-Location $repoRoot
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Warning "cargo not found in PATH. Install Rust toolchain first or run this script from a developer system with cargo available."
} else {
    & cargo build --release
}
Pop-Location

# 2) Download rustup-init.exe
$rustupUrl = 'https://win.rustup.rs/x86_64'
$rustupTarget = Join-Path $toolsDir 'rustup-init.exe'
if (-not (Test-Path $rustupTarget)) {
    Write-Host "Downloading rustup-init.exe from $rustupUrl ..."
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupTarget -UseBasicParsing -ErrorAction Stop
        Write-Host "Saved rustup-init.exe to $rustupTarget"
    } catch {
        Write-Warning "Failed to download rustup-init.exe: $_"
        Write-Warning "You can manually place rustup-init.exe at $rustupTarget"
    }
} else { Write-Host "rustup-init.exe already present at $rustupTarget" }

# 3) Attempt to download avrdude.exe (best-effort). Replace or provide manually if download fails.
$avrdudeTarget = Join-Path $toolsDir 'avrdude.exe'
$avrdudeCandidates = @(
    'https://example.com/avrdude.exe' # <- Replace with a real avrdude Windows binary URL if you have one
)
if (-not (Test-Path $avrdudeTarget)) {
    foreach ($url in $avrdudeCandidates) {
        try {
            Write-Host "Attempting to download avrdude from $url ..."
            Invoke-WebRequest -Uri $url -OutFile $avrdudeTarget -UseBasicParsing -ErrorAction Stop
            Write-Host "Saved avrdude.exe to $avrdudeTarget"
            break
        } catch {
            Write-Warning "Failed to download from $url: $_"
        }
    }
    if (-not (Test-Path $avrdudeTarget)) {
        Write-Warning "avrdude.exe was not downloaded. Please provide a Windows avrdude binary at: $avrdudeTarget"
    }
} else { Write-Host "avrdude.exe already present at $avrdudeTarget" }

# 4) Run Inno Setup Compiler (ISCC.exe)
# Common install locations for Inno Setup 6
$isccPaths = @( 
    "$Env:ProgramFiles(x86)\\Inno Setup 6\\ISCC.exe",
    "$Env:ProgramFiles\\Inno Setup 6\\ISCC.exe"
)
$isccPath = $isccPaths | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $isccPath) {
    Write-Warning "ISCC.exe not found in common locations. Please install Inno Setup 6 and ensure ISCC.exe is available."
    Write-Host "You can also run ISCC manually: ISCC.exe $scriptDir\\oxide_setup.iss"
    exit 0
}

Write-Host "Found ISCC: $isccPath"
& "$isccPath" (Join-Path $scriptDir 'oxide_setup.iss')

Write-Host "Installer build finished. Output will be in: $outputDir (per oxide_setup.iss)"

# End
