# ChatImproVR Demo-Room

This is a "demo room" plugin that includes a room that you can walk around in. 


## Preparation

Make sure you have the `wasm32-unknown-unknown` target installed:
```sh
rustup target add wasm32-unknown-unknown
```

## Building
Now you can use `cargo build --release` to build. Your plugin will show up under `target/wasm32-unknown-unknown/release/demo_room.wasm`.

## Running
Assuming you have set up `cimvr` scripts, you can use this plugin by simply running:

```sh
cimvr camera demo_room
```

## Testing
Because `.cargo/config.toml` is set up to compile for the WASM target, tests will fail to run by default. You can compile and run tests using the provided `test_pc` alias:
```sh
cargo test_pc
```
