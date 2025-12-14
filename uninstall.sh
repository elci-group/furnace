#!/bin/bash
# Furnace Uninstaller

set -e

echo "🔥 Furnace Uninstaller"
echo "======================"
echo ""

# Find where furnace is installed
FURNACE_PATH=$(which furnace 2>/dev/null || echo "")

if [ -z "$FURNACE_PATH" ]; then
    echo "❌ Furnace is not installed or not in PATH"
    exit 1
fi

echo "Found furnace at: $FURNACE_PATH"
echo ""

# Determine if we need sudo
SUDO=""
if [[ "$FURNACE_PATH" == "/usr/local/bin/"* ]] || [[ "$FURNACE_PATH" == "/usr/bin/"* ]]; then
    if [ "$EUID" -ne 0 ]; then
        SUDO="sudo"
        echo "🔐 Requesting sudo permission to uninstall..."
    fi
fi

echo "🗑️  Removing furnace..."
$SUDO rm -f "$FURNACE_PATH"

if [ $? -eq 0 ]; then
    echo "✅ Furnace has been uninstalled!"
else
    echo "❌ Uninstallation failed!"
    exit 1
fi
