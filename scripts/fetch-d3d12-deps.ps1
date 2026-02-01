# Fetch D3D12 NuGet dependencies for Windows development
# Usage: .\scripts\fetch-d3d12-deps.ps1
#
# This script downloads the required NuGet packages for D3D12 development:
# - DXC (DirectX Shader Compiler) - for runtime shader compilation
# - DirectStorage - for fast asset loading
#
# Note: D3D12/DXGI headers and libraries come from the Windows SDK which
# is already installed on Windows development machines and GitHub Actions runners.

param(
    [string]$OutputDir = "$PSScriptRoot\..\packages"
)

$ErrorActionPreference = "Stop"

# Package versions (update these when upgrading)
$DXC_VERSION = "1.8.2505.32"
$DSTORAGE_VERSION = "1.3.0"

Write-Host "Fetching D3D12 dependencies to: $OutputDir" -ForegroundColor Cyan

# Create output directory if it doesn't exist
if (!(Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

# Fetch packages
Write-Host "Installing Microsoft.Direct3D.DXC $DXC_VERSION..." -ForegroundColor Yellow
nuget install Microsoft.Direct3D.DXC -Version $DXC_VERSION -OutputDirectory $OutputDir

Write-Host "Installing Microsoft.Direct3D.DirectStorage $DSTORAGE_VERSION..." -ForegroundColor Yellow
nuget install Microsoft.Direct3D.DirectStorage -Version $DSTORAGE_VERSION -OutputDirectory $OutputDir

# Resolve absolute paths
$OutputDir = (Resolve-Path $OutputDir).Path
$DXC_PATH = "$OutputDir\Microsoft.Direct3D.DXC.$DXC_VERSION"
$DSTORAGE_PATH = "$OutputDir\Microsoft.Direct3D.DirectStorage.$DSTORAGE_VERSION"

Write-Host "`nPackages installed successfully!" -ForegroundColor Green
Write-Host "`nTo build D3D12 targets, set these environment variables:" -ForegroundColor Cyan
Write-Host "  `$env:DXC_PATH = `"$DXC_PATH`""
Write-Host "  `$env:DSTORAGE_PATH = `"$DSTORAGE_PATH`""

# If running in GitHub Actions, export to GITHUB_ENV
if ($env:GITHUB_ENV) {
    Write-Host "`nExporting to GITHUB_ENV..." -ForegroundColor Yellow
    echo "DXC_PATH=$DXC_PATH" >> $env:GITHUB_ENV
    echo "DSTORAGE_PATH=$DSTORAGE_PATH" >> $env:GITHUB_ENV
}

# Output paths for scripting
Write-Host "`n# Copy-paste to set environment variables for this session:" -ForegroundColor Magenta
Write-Host "`$env:DXC_PATH = `"$DXC_PATH`"; `$env:DSTORAGE_PATH = `"$DSTORAGE_PATH`""
