$ErrorActionPreference = "Stop"

$BinDir = "$env:USERPROFILE\AppData\Local\bin"
$BinaryPath = "$BinDir\commitz.exe"

if (!(Test-Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir | Out-Null
}

if (Test-Path $BinaryPath) {
    Write-Host "🗑️ Removing old commitz.exe..."
    Remove-Item $BinaryPath -Force
}

Write-Host "⬇️ Downloading latest commitz.exe..."
Invoke-WebRequest -Uri "https://github.com/rynsh1506/commitz/releases/download/v0.1.0/commitz.exe" -OutFile $BinaryPath

# Tambah PATH kalau belum ada
if (-not ($env:Path -split ";" | ForEach-Object { $_.Trim() } | Where-Object { $_ -eq $BinDir })) {
    setx PATH "$env:PATH;$BinDir"
    Write-Host "🔧 Added $BinDir to PATH. Restart terminal or open a new window."
}

Write-Host "✅ commitz installed! Run: commitz"
