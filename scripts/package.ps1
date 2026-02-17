# package.ps1
# Creates a distributable ZIP for PDFTool.
# Run after: cargo build --release -p pdftool-gui
#
# Output: PDFTool-windows-x64.zip

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path $PSScriptRoot
$releaseDir  = Join-Path $projectRoot "target\release"
$stageDir    = Join-Path $projectRoot "target\package\PDFTool"
$zipOutput   = Join-Path $projectRoot "target\package\PDFTool-windows-x64.zip"

# --- Validate ---
$exe = Join-Path $releaseDir "pdftool-gui.exe"
$gs  = Join-Path $releaseDir "ghostscript"

if (-not (Test-Path $exe)) {
    Write-Error "Binary not found at $exe. Run 'cargo build --release -p pdftool-gui' first."
    exit 1
}
if (-not (Test-Path $gs)) {
    Write-Error "Bundled Ghostscript not found at $gs. Run '.\scripts\setup-ghostscript.ps1' first."
    exit 1
}

# --- Stage ---
Write-Host "Packaging PDFTool..." -ForegroundColor Cyan

if (Test-Path $stageDir) { Remove-Item $stageDir -Recurse -Force }
New-Item -ItemType Directory -Path $stageDir -Force | Out-Null

# Copy binary (rename to PDFTool.exe for cleanliness)
Copy-Item $exe (Join-Path $stageDir "PDFTool.exe") -Force

# Copy icon
$icon = Join-Path $projectRoot "src-tauri\icons\icon.ico"
if (Test-Path $icon) {
    Copy-Item $icon (Join-Path $stageDir "PDFTool.ico") -Force
}

# Copy bundled Ghostscript
Copy-Item $gs (Join-Path $stageDir "ghostscript") -Recurse -Force

# Copy bundled Pandoc+Typst (if available)
$pandoc = Join-Path $releaseDir "pandoc"
if (Test-Path $pandoc) {
    Copy-Item $pandoc (Join-Path $stageDir "pandoc") -Recurse -Force
    Write-Host "  Pandoc+Typst bundle included" -ForegroundColor Green
} else {
    Write-Host "  Warning: Pandoc+Typst bundle not found. Markdown->PDF won't work standalone." -ForegroundColor Yellow
}

# Copy pdftotext + required DLLs (for text extraction & PDF->Markdown)
$pdftextSrc = "C:\Program Files\Git\mingw64\bin\pdftotext.exe"
if (Test-Path $pdftextSrc) {
    Copy-Item $pdftextSrc (Join-Path $stageDir "pdftotext.exe") -Force
    # Copy required mingw64 DLLs
    $dllDir = "C:\Program Files\Git\mingw64\bin"
    $dlls = @("libgcc_s_seh-1.dll", "libstdc++-6.dll", "libwinpthread-1.dll", "zlib1.dll")
    foreach ($dll in $dlls) {
        $dllPath = Join-Path $dllDir $dll
        if (Test-Path $dllPath) {
            Copy-Item $dllPath (Join-Path $stageDir $dll) -Force
        }
    }
    Write-Host "  pdftotext + DLLs included" -ForegroundColor Green
} else {
    Write-Host "  Warning: pdftotext not found. Text extraction & PDF->Markdown won't work standalone." -ForegroundColor Yellow
}

# --- Zip ---
if (Test-Path $zipOutput) { Remove-Item $zipOutput -Force }
Compress-Archive -Path $stageDir -DestinationPath $zipOutput -CompressionLevel Optimal

$sizeMB = [math]::Round((Get-Item $zipOutput).Length / 1MB, 1)
Write-Host ""
Write-Host "Package created: $zipOutput ($sizeMB MB)" -ForegroundColor Green
Write-Host "Ready to upload as a GitHub Release asset." -ForegroundColor Yellow
