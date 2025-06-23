if (-not $IsWindows) {
    Write-Error "❌ This script only works on Windows."
    exit 1
}

$repo = "Lunarr199/auto-clicker"
$asset = "auto-clicker-x86_64-pc-windows-gnu.zip"
$url = "https://github.com/$repo/releases/latest/download/$asset"

# Safe paths
$zipPath = "$env:TEMP\auto-clicker.zip"
$tempDir = New-Item -ItemType Directory -Path ([System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.IO.Path]::GetRandomFileName()))
$installDir = "C:\Program Files\auto-clicker"
$finalExeName = "auto-clicker.exe"
$finalPath = Join-Path $installDir $finalExeName

Write-Host "📦 Downloading latest release..."
Invoke-WebRequest -Uri $url -OutFile $zipPath

Write-Host "📂 Extracting zip..."
Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

$extractedExe = Get-ChildItem -Path $tempDir -Filter *.exe -Recurse | Select-Object -First 1

if (-not $extractedExe) {
    Write-Error "❌ No .exe found in the archive."
    exit 1
}

# Ensure install dir exists
if (-not (Test-Path $installDir)) {
    New-Item -Path $installDir -ItemType Directory | Out-Null
}

# Overwrite existing
if (Test-Path $finalPath) {
    Remove-Item $finalPath -Force
}

Move-Item -Path $extractedExe.FullName -Destination $finalPath

Write-Host "✅ Installed to $finalPath"

# Safely update PATH
$envPath = [Environment]::GetEnvironmentVariable("Path", "User")
$envPathList = if ($envPath) { $envPath.Split(";") } else { @() }

# Normalize to prevent trailing slashes from breaking string match
$normalizedEnvPathList = $envPathList | ForEach-Object { $_.TrimEnd("\") }
$normalizedInstallDir = $installDir.TrimEnd("\")

if (-not ($normalizedEnvPathList -contains $normalizedInstallDir)) {
    $newPath = if ($envPath) { "$envPath;$installDir" } else { $installDir }
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "🔧 Added '$installDir' to user PATH. You may need to restart your shell."
} else {
    Write-Host "ℹ️ '$installDir' is already in PATH."
}

# Clean up
Remove-Item $zipPath -Force
Remove-Item $tempDir -Recurse -Force

Write-Host "`n🚀 To run it from anywhere, type:"
Write-Host "`n  auto-clicker --help`n"
