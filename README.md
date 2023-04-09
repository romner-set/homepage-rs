## Code for my [humble website](https://myalpine.live)

Styling & turbofish animations are from https://github.com/jplatte/turbo.fish. The code itself was completely rewritten in [https://yew.rs](yew), because why write HTML when you can write Rust?

### Commands to run:
```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
trunk serve --release --open
```
