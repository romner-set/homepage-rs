## Code for my [humble website](https://myalpine.live)

Styling & turbofish animations are from https://github.com/jplatte/turbo.fish. The code itself was completely rewritten in [https://yew.rs](https://yew.rs), because why write HTML when you can write Rust?

### Commands to run a dev server:
```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
trunk serve --release --open
```

You can also just serve the static files in `dist/` after a `trunk build --release` with any other webserver. Installing `wasm-opt` from [binaryen](https://github.com/WebAssembly/binaryen) to optimize the final .wasm is also recommended.
