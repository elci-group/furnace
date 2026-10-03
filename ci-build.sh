#!/bin/bash
# CI-friendly build script with retry logic and network resilience

set -euo pipefail

# Configuration
MAX_RETRIES=3
RETRY_DELAY=5
OFFLINE_BUILD=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to retry a command with exponential backoff
retry_command() {
    local cmd="$1"
    local retries=0
    local delay=$RETRY_DELAY
    
    while [ $retries -lt $MAX_RETRIES ]; do
        log_info "Attempting: $cmd (attempt $((retries + 1))/$MAX_RETRIES)"
        
        if eval "$cmd"; then
            log_info "Command succeeded!"
            return 0
        fi
        
        retries=$((retries + 1))
        
        if [ $retries -lt $MAX_RETRIES ]; then
            log_warn "Command failed, retrying in ${delay}s..."
            sleep $delay
            delay=$((delay * 2)) # Exponential backoff
        fi
    done
    
    log_error "Command failed after $MAX_RETRIES attempts: $cmd"
    return 1
}

# Check if we can use offline build
if [ -d "vendor" ] && [ -f ".cargo/config.toml" ]; then
    log_info "Vendor directory found, attempting offline build first..."
    
    if cargo build --offline --release; then
        log_info "✅ Offline build successful!"
        OFFLINE_BUILD=true
    else
        log_warn "Offline build failed, falling back to online build with retries..."
    fi
fi

# If offline build didn't work, try online build with retries
if [ "$OFFLINE_BUILD" = false ]; then
    log_info "Attempting online build with retry logic..."
    
    # Update index with retry
    retry_command "cargo update"
    
    # Build with retry
    retry_command "cargo build --release"
fi

# Run tests if build succeeded
if [ $? -eq 0 ]; then
    log_info "Build successful, running tests..."
    
    if [ "$OFFLINE_BUILD" = true ]; then
        cargo test --offline
    else
        retry_command "cargo test"
    fi
fi

log_info "✅ CI build completed successfully!"