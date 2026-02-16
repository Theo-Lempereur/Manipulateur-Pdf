# setup-ghostscript.ps1
# Copies the required Ghostscript files from a system installation
# into the project's ghostscript/ folder for bundling.
#
# Prerequisites: Ghostscript must be installed on the build machine.
# Download from: https://www.ghostscript.com/releases/gsdnld.html

param(
    [string]$GsPath = ""
)

$ErrorActionPreference = "Stop"

# Auto-detect Ghostscript installation
if (-not $GsPath) {
    $gsExe = Get-Command "gswin64c" -ErrorAction SilentlyContinue
    if ($gsExe) {
        $GsPath = Split-Path (Split-Path $gsExe.Source)
    } else {
        # Try common installation paths
        $candidates = Get-ChildItem "C:\Program Files\gs" -ErrorAction SilentlyContinue |
            Sort-Object Name -Descending |
            Select-Object -First 1
        if ($candidates) {
            $GsPath = $candidates.FullName
        }
    }
}

if (-not $GsPath -or -not (Test-Path $GsPath)) {
    Write-Error "Ghostscript installation not found. Install it or pass -GsPath <path>"
    exit 1
}

Write-Host "Using Ghostscript from: $GsPath" -ForegroundColor Cyan

$projectRoot = Split-Path $PSScriptRoot
$dest = Join-Path $projectRoot "ghostscript"

# Clean previous copy
if (Test-Path $dest) {
    Remove-Item $dest -Recurse -Force
}

# Copy required files
New-Item -ItemType Directory -Path "$dest\bin" -Force | Out-Null
Copy-Item "$GsPath\bin\gswin64c.exe" "$dest\bin\" -Force
Copy-Item "$GsPath\bin\gsdll64.dll"  "$dest\bin\" -Force
Copy-Item "$GsPath\lib"              "$dest\lib"         -Recurse -Force
Copy-Item "$GsPath\Resource"         "$dest\Resource"    -Recurse -Force
Copy-Item "$GsPath\iccprofiles"      "$dest\iccprofiles" -Recurse -Force

$sizeMB = [math]::Round(
    (Get-ChildItem $dest -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB, 1
)

Write-Host "Ghostscript bundled to $dest ($sizeMB MB)" -ForegroundColor Green
Write-Host "You can now build with: cargo build --release -p pdftool-gui" -ForegroundColor Yellow
