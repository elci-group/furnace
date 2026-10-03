#!/bin/bash
# Vendor script for furnace - bundles critical dependencies for offline builds
# Run this script before CI to ensure dependencies are available offline

set -euo pipefail

echo "🔧 Vendoring dependencies for furnace..."

# Create vendor directory if it doesn't exist
mkdir -p vendor

# Vendor only critical dependencies that are likely to fail due to network issues
echo "📦 Vendoring core dependencies..."
cargo vendor --versioned-dirs --respect-source-config vendor/ \
  --sync Cargo.toml

# Create a .cargo/config.toml for using the vendor directory
cat > .cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

# Fallback to crates.io if vendor fails
[source.crates-io-fallback]
registry = "https://github.com/rust-lang/crates.io-index"
EOF

echo "✅ Vendoring complete!"
echo "📂 Dependencies stored in: vendor/"
echo "🔧 CI can now use: cargo build --offline"

# Verify offline build works
echo "🧪 Testing offline build..."
cargo build --offline --release

echo "✅ Offline build successful!"