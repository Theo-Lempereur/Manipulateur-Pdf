<#
.SYNOPSIS
    Install PDFTool — PDF manipulation made simple.

.DESCRIPTION
    Downloads and installs PDFTool to your local machine.
    Creates a Start Menu shortcut so you can launch it by pressing
    Win, typing "PDFTool", then Enter.

.EXAMPLE
    iwr https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/install.ps1 | iex
#>

$ErrorActionPreference = "Stop"

# ── Config ──────────────────────────────────────────────────────────
$AppName    = "PDFTool"
$RepoOwner  = "Theo-Lempereur"
$RepoName   = "Manipulateur-Pdf"
$ZipName    = "PDFTool-windows-x64.zip"
$InstallDir = Join-Path $env:LOCALAPPDATA $AppName
$StartMenu  = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"

# ── Banner ──────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ╔═══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "  ║                                           ║" -ForegroundColor Cyan
Write-Host "  ║      ██████  ██████  ███████ ████████     ║" -ForegroundColor Cyan
Write-Host "  ║      ██   ██ ██   ██ ██         ██        ║" -ForegroundColor Cyan
Write-Host "  ║      ██████  ██   ██ █████      ██        ║" -ForegroundColor Cyan
Write-Host "  ║      ██      ██   ██ ██         ██        ║" -ForegroundColor Cyan
Write-Host "  ║      ██      ██████  ██         ██        ║" -ForegroundColor Cyan
Write-Host "  ║                                           ║" -ForegroundColor Cyan
Write-Host "  ║       PDF manipulation made simple        ║" -ForegroundColor DarkCyan
Write-Host "  ║                                           ║" -ForegroundColor Cyan
Write-Host "  ╚═══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ── Find latest release ────────────────────────────────────────────
Write-Host "  [1/4] Finding latest release..." -ForegroundColor White
$apiUrl  = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing
} catch {
    Write-Host "  ERROR: Could not reach GitHub. Check your internet connection." -ForegroundColor Red
    exit 1
}

$asset = $release.assets | Where-Object { $_.name -eq $ZipName } | Select-Object -First 1
if (-not $asset) {
    Write-Host "  ERROR: Release asset '$ZipName' not found." -ForegroundColor Red
    Write-Host "         Visit https://github.com/$RepoOwner/$RepoName/releases" -ForegroundColor DarkGray
    exit 1
}

$version     = $release.tag_name
$downloadUrl = $asset.browser_download_url
Write-Host "  Found $AppName $version" -ForegroundColor Green

# ── Download ────────────────────────────────────────────────────────
Write-Host "  [2/4] Downloading ($([math]::Round($asset.size / 1MB, 1)) MB)..." -ForegroundColor White
$tempDir = [System.IO.Path]::GetTempPath()
$tempZip = Join-Path $tempDir "$AppName-install.zip"
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing

# ── Install ─────────────────────────────────────────────────────────
Write-Host "  [3/4] Installing to $InstallDir..." -ForegroundColor White

# Clean previous installation
if (Test-Path $InstallDir) {
    # Kill running instance if any
    Stop-Process -Name "PDFTool" -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 500
    Remove-Item $InstallDir -Recurse -Force
}

# Extract zip (contains a PDFTool/ folder inside)
$tempExtract = Join-Path $tempDir "$AppName-extract"
try { if (Test-Path $tempExtract) { Remove-Item $tempExtract -Recurse -Force } } catch {}
Expand-Archive -Path $tempZip -DestinationPath $tempExtract -Force

# Find the PDFTool folder inside the extracted content
$extracted = Get-ChildItem $tempExtract -Directory | Select-Object -First 1
if ($extracted) {
    Move-Item $extracted.FullName $InstallDir -Force
} else {
    # Files are directly in the extract folder
    Move-Item $tempExtract $InstallDir -Force
}

# Cleanup temp files
try { Remove-Item $tempZip -Force } catch {}
try { Remove-Item $tempExtract -Recurse -Force } catch {}

# Verify
$exePath = Join-Path $InstallDir "PDFTool.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "  ERROR: Installation failed — PDFTool.exe not found." -ForegroundColor Red
    exit 1
}

# ── Start Menu shortcut ────────────────────────────────────────────
Write-Host "  [4/4] Creating Start Menu shortcut..." -ForegroundColor White
$ws = New-Object -ComObject WScript.Shell
$shortcut = $ws.CreateShortcut("$StartMenu\$AppName.lnk")
$shortcut.TargetPath       = $exePath
$shortcut.WorkingDirectory = $InstallDir
$shortcut.Description      = "PDFTool — PDF manipulation made simple"

# Set icon if available
$iconPath = Join-Path $InstallDir "PDFTool.ico"
if (Test-Path $iconPath) {
    $shortcut.IconLocation = "$iconPath,0"
}
$shortcut.Save()

# ── Done ────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ✓ PDFTool $version installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "  How to launch:" -ForegroundColor White
Write-Host "    • Press Win, type `"PDFTool`", press Enter" -ForegroundColor DarkCyan
Write-Host "    • Or run: $exePath" -ForegroundColor DarkGray
Write-Host ""
Write-Host "  To uninstall later:" -ForegroundColor White
Write-Host "    iwr https://raw.githubusercontent.com/$RepoOwner/$RepoName/main/uninstall.ps1 | iex" -ForegroundColor DarkGray
Write-Host ""
