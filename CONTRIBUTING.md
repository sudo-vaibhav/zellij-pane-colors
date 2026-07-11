# Contributing

Thanks for helping improve zellij-pane-colors.

## Development

Install Rust with rustup and add the WebAssembly target:

```sh
rustup target add wasm32-wasip1
cargo fmt --check
cargo clippy --target wasm32-wasip1 -- -D warnings
cargo build --release --target wasm32-wasip1
```

Test plugin changes in a disposable Zellij session before changing your default
configuration. At minimum, verify initial, tiled, floating, stacked, and command
panes; confirm focus, resize, and rename do not recolour existing panes; and
confirm plugin panes remain unchanged.

## Pull requests

- Keep the plugin small and dependency-light.
- Avoid polling, external daemons, and terminal escape injection.
- Preserve existing pane backgrounds when the API exposes them.
- Explain user-visible behavior changes and update the README.
- Ensure formatting, Clippy, and the release build pass.

By participating, you agree to follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

