# setup-pandoc.ps1
# Downloads/copies Pandoc and Typst into the project's pandoc/ folder for bundling.
# Same pattern as setup-ghostscript.ps1.
#
# Prerequisites: Pandoc and Typst must be installed on the build machine.

param(
    [string]$PandocPath = "",
    [string]$TypstPath  = ""
)

$ErrorActionPreference = "Stop"

# ── Auto-detect Pandoc ──────────────────────────────────────────────
if (-not $PandocPath) {
    # 1. Check PATH
    $cmd = Get-Command "pandoc" -ErrorAction SilentlyContinue
    if ($cmd) {
        $PandocPath = $cmd.Source
    } else {
        # 2. Known install location
        $known = "$env:LOCALAPPDATA\Pandoc\pandoc.exe"
        if (Test-Path $known) { $PandocPath = $known }
    }
}

if (-not $PandocPath -or -not (Test-Path $PandocPath)) {
    Write-Error "Pandoc not found. Install it (winget install JohnMacFarlane.Pandoc) or pass -PandocPath <path>"
    exit 1
}

Write-Host "Using Pandoc: $PandocPath" -ForegroundColor Cyan

# ── Auto-detect Typst ───────────────────────────────────────────────
if (-not $TypstPath) {
    # 1. Check PATH
    $cmd = Get-Command "typst" -ErrorAction SilentlyContinue
    if ($cmd) {
        $TypstPath = $cmd.Source
    } else {
        # 2. WinGet packages folder
        $wingetPkg = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages"
        if (Test-Path $wingetPkg) {
            $found = Get-ChildItem $wingetPkg -Recurse -Filter "typst.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($found) { $TypstPath = $found.FullName }
        }
    }
}

if (-not $TypstPath -or -not (Test-Path $TypstPath)) {
    Write-Error "Typst not found. Install it (winget install typst) or pass -TypstPath <path>"
    exit 1
}

Write-Host "Using Typst:  $TypstPath" -ForegroundColor Cyan

# ── Copy to project ────────────────────────────────────────────────
$projectRoot = Split-Path $PSScriptRoot
$dest = Join-Path $projectRoot "pandoc"

# Clean previous copy
if (Test-Path $dest) {
    Remove-Item $dest -Recurse -Force
}

New-Item -ItemType Directory -Path $dest -Force | Out-Null

Copy-Item $PandocPath (Join-Path $dest "pandoc.exe") -Force
Copy-Item $TypstPath  (Join-Path $dest "typst.exe")  -Force

$sizeMB = [math]::Round(
    (Get-ChildItem $dest -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB, 1
)

Write-Host ""
Write-Host "Pandoc + Typst bundled to $dest ($sizeMB MB)" -ForegroundColor Green
Write-Host "You can now build with: cargo build --release -p pdftool-gui" -ForegroundColor Yellow
