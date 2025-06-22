#!/bin/bash
set -e

ZIP_NAME="auto-clicker-x86_64-unknown-linux-gnu.zip"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="$(mktemp -d)"
FINAL_NAME="auto-clicker" 

echo "🔍 Fetching latest release info..."
URL=$(curl -s "https://api.github.com/repos/Lunarr199/auto-clicker/releases/latest" \
  | grep "browser_download_url" \
  | grep "$ZIP_NAME" \
  | cut -d '"' -f 4)

if [ -z "$URL" ]; then
  echo "❌ Could not find a release asset for '$ZIP_NAME'"
  exit 1
fi

echo "⬇️  Downloading from $URL"
curl -L "$URL" -o "$ZIP_NAME"

echo "📦 Inspecting contents..."
EXTRACTED_FILE=$(unzip -Z1 "$ZIP_NAME" | head -n 1)

echo "🧹 Removing old $INSTALL_DIR/$FINAL_NAME (if exists)..."
if [ -f "$INSTALL_DIR/$FINAL_NAME" ]; then
  sudo rm -f "$INSTALL_DIR/$FINAL_NAME"
fi

echo "📂 Extracting $ZIP_NAME..."
unzip -q "$ZIP_NAME" -d "$TEMP_DIR"

echo "🚀 Installing $FINAL_NAME..."
chmod +x "$TEMP_DIR/$EXTRACTED_FILE"
sudo mv "$TEMP_DIR/$EXTRACTED_FILE" "$INSTALL_DIR/$FINAL_NAME"

echo "✅ Installed as $INSTALL_DIR/$FINAL_NAME"

rm -rf "$TEMP_DIR"
rm -f "$ZIP_NAME"

auto-clicker --help