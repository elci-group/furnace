#!/bin/bash
# Furnace Installer
# Builds and installs furnace to /usr/local/bin

set -e

echo "🔥 Furnace Installer"
echo "===================="
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -w "/usr/bin" ]; then
    INSTALL_DIR="/usr/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    echo "⚠️  Installing to $INSTALL_DIR (add to PATH if needed)"
fi

echo "📦 Building Furnace (release mode)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Check if we need sudo
SUDO=""
if [ "$INSTALL_DIR" = "/usr/local/bin" ] || [ "$INSTALL_DIR" = "/usr/bin" ]; then
    if [ "$EUID" -ne 0 ]; then
        SUDO="sudo"
        echo "🔐 Requesting sudo permission to install to $INSTALL_DIR..."
    fi
fi

echo "📥 Installing to $INSTALL_DIR..."
$SUDO cp target/release/furnace "$INSTALL_DIR/furnace"
$SUDO chmod +x "$INSTALL_DIR/furnace"

if [ $? -eq 0 ]; then
    echo "✅ Installation successful!"
    echo ""
    echo "🎉 Furnace is now installed!"
    echo ""
    echo "Try it out:"
    echo "  furnace --help"
    echo "  furnace . --tree"
    echo "  furnace . --layman=openai  # Requires AI features"
    echo ""
    
    if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
        echo "⚠️  Note: Make sure $HOME/.local/bin is in your PATH:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo "  (Add to your ~/.bashrc or ~/.zshrc)"
        echo ""
    fi
    
    echo "📚 Documentation: https://github.com/elci-group/furnace"
else
    echo "❌ Installation failed!"
    exit 1
fi
