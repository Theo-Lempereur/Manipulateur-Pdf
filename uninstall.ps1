<#
.SYNOPSIS
    Uninstall PDFTool.

.EXAMPLE
    iwr https://raw.githubusercontent.com/Theo-Lempereur/Manipulateur-Pdf/main/uninstall.ps1 | iex
#>

$ErrorActionPreference = "Stop"

$AppName    = "PDFTool"
$InstallDir = Join-Path $env:LOCALAPPDATA $AppName
$StartMenu  = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
$Shortcut   = Join-Path $StartMenu "$AppName.lnk"

Write-Host ""
Write-Host "  Uninstalling $AppName..." -ForegroundColor Cyan

# Kill running instance
Stop-Process -Name "PDFTool" -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500

# Remove shortcut
if (Test-Path $Shortcut) {
    Remove-Item $Shortcut -Force
    Write-Host "  Removed Start Menu shortcut" -ForegroundColor DarkGray
}

# Remove install directory
if (Test-Path $InstallDir) {
    Remove-Item $InstallDir -Recurse -Force
    Write-Host "  Removed $InstallDir" -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "  ✓ $AppName has been uninstalled." -ForegroundColor Green
Write-Host ""
