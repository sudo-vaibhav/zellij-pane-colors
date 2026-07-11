#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
install_dir="${XDG_CONFIG_HOME:-$HOME/.config}/zellij/plugins"

if ! command -v cargo >/dev/null 2>&1; then
  printf 'error: cargo is required (install Rust from https://rustup.rs)\n' >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx 'wasm32-wasip1'; then
  printf 'error: missing Rust target; run: rustup target add wasm32-wasip1\n' >&2
  exit 1
fi

cargo build --manifest-path "$project_dir/Cargo.toml" --release --target wasm32-wasip1
install -d "$install_dir"
install -m 0644 \
  "$project_dir/target/wasm32-wasip1/release/zellij-pane-colors.wasm" \
  "$install_dir/zellij-pane-colors.wasm"

printf 'Installed %s\n' "$install_dir/zellij-pane-colors.wasm"
printf 'See README.md for the additive Zellij configuration entries.\n'
