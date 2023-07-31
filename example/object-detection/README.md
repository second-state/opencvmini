# example: object detection

## Run this repository

By the command

```shell
cargo build --target wasm32-wasi --release
WASMEDGE_PLUGIN_PATH=/usr/local/lib/wasmedge/ wasmedge --dir ./. target/wasm32-wasi/release/plugin-example.wasm
```

or prepared `make`
