# opencvmini

Rust SDK for WasmEdge Plugin [OpenCVMini](https://github.com/WasmEdge/WasmEdge/tree/master/plugins/wasmedge_opencvmini)

## Usage

Add dependency into `Cargo.toml`

```toml
opencvmini = "*"
```

then you can use this crate

```rust
use opencvmini::*;
```

## Dependencies

[opencvmini](https://github.com/WasmEdge/WasmEdge/tree/master/plugins/wasmedge_opencvmini) is a wasmedge plugin, you might like to clone repository [WasmEdge](https://github.com/WasmEdge/WasmEdge), and run the following commands to install this plugin.

```shell
# In repository wasmedge
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_PLUGINS=ON \
  -DWASMEDGE_PLUGIN_OPENCVMINI=ON \
  -DWASMEDGE_PLUGIN_TENSORFLOWLITE=ON \
  -DWASMEDGE_PLUGIN_IMAGE=ON \
  -GNinja ..
ninja
ninja install # might need `sudo`
```

You also need command-line tool `witc` to generate code from WIT format, please refer to [installation section](https://github.com/second-state/witc#installation) to get it.
