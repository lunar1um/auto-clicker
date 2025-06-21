$repo = "Lunarr199/auto-clicker"
$asset = "auto-clicker-x86_64-pc-windows-gnu.zip"
$url = "https://github.com/$repo/releases/latest/download/$asset"
$zipPath = "$env:TEMP\auto-clicker.zip"
$tempDir = New-Item -ItemType Directory -Path ([System.IO.Path]::GetTempPath() + [System.IO.Path]::GetRandomFileName())
$installDir = "C:\Program Files\auto-clicker"
$finalExeName = "auto-clicker.exe"
$finalPath = Join-Path $installDir $finalExeName

Write-Host "📦 Downloading latest release..."
Invoke-WebRequest -Uri $url -OutFile $zipPath

Write-Host "📂 Extracting zip..."
Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

# Detect .exe file in extracted contents
$extractedExe = Get-ChildItem -Path $tempDir -Filter *.exe -Recurse | Select-Object -First 1

if (-not $extractedExe) {
    Write-Error "❌ No .exe found in the archive."
    exit 1
}

# Create install dir if not exists
if (-not (Test-Path $installDir)) {
    New-Item -Path $installDir -ItemType Directory | Out-Null
}

# Remove old install if it exists
if (Test-Path $finalPath) {
    Remove-Item $finalPath -Force
}

# Rename and move binary
Move-Item -Path $extractedExe.FullName -Destination $finalPath

Write-Host "✅ Installed to $finalPath"

# Add to PATH if not already
$envPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $envPath.Split(";") -contains $installDir) {
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$installDir", "User")
    Write-Host "🔧 Added '$installDir' to user PATH. You may need to restart your shell."
} else {
    Write-Host "ℹ️ '$installDir' is already in PATH."
}

# Clean up
Remove-Item $zipPath -Force
Remove-Item $tempDir -Recurse -Force

Write-Host "`n🚀 To run it from anywhere, type:"
Write-Host "`n  auto-clicker --help`n"
