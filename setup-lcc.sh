#!/bin/bash
set -e

echo "=== Linux Control Center Setup ==="

# Create project structure
mkdir -p linux-control-center/{apps/desktop/{src/{components/{layout,search,ui},pages,hooks,stores,types,lib},public,src-tauri/{src,icons}},crates/{core,plugin-api,package-engine,service-engine,hardware-engine,storage-engine,network-engine,security-engine,logs-engine,backup-engine},plugins/{apt,pacman,dnf,flatpak,snap,systemd,docker},shared/{types,ui,utilities},database,docs,tests,scripts,.github/workflows}

cd linux-control-center

# === ROOT FILES ===

cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/plugin-api",
    "crates/package-engine",
    "crates/service-engine",
    "crates/hardware-engine",
    "crates/storage-engine",
    "crates/network-engine",
    "crates/security-engine",
    "crates/logs-engine",
    "crates/backup-engine",
    "apps/desktop/src-tauri",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Linux Control Center Contributors"]
license = "GPL-3.0"
repository = "https://github.com/linux-control-center/lcc"
rust-version = "1.75"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full", "tracing"] }
tokio-util = { version = "0.7", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "migrate", "chrono"] }
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
zbus = { version = "4.0", features = ["tokio"] }
chrono = { version = "0.4", features = ["serde"] }
config = "0.14"
toml = "0.8"
uuid = { version = "1.6", features = ["v4", "serde"] }
regex = "1.10"
lazy_static = "1.4"
dashmap = "5.5"
parking_lot = "0.12"
indexmap = { version = "2.1", features = ["serde"] }
mockall = "0.12"
tempfile = "3.8"
tauri = { version = "2.0", features = ["tray-icon", "notification"] }
tauri-build = "2.0"
async-trait = "0.1"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
EOF

cat > README.md << 'EOF'
# Linux Control Center (LCC)

A modern, extensible Linux system management suite.

## Quick Start

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev nodejs npm

# Build and run
./scripts/build.sh
./scripts/run.sh
