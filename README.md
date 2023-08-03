# cuda-driver-sys

Rust binding to CUDA driver library (```libcuda.so```)

This project focuses only ```cuda driver api```, the full cuda-sys can see [```cuda-sys```]

### Usage

1. Just one version, such as cuda version 10.2.

```toml
[dependencies]
cuda-driver-sys = { version = "0.3", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-102" }
```

2. More than one version with cargo features.

```toml
[dependencies]
cuda-driver-sys-102 = { package = "cuda-driver-sys", version = "0.3", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-102", optional = true }
cuda-driver-sys-114 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-114", optional = true }

[features]
default-cuda-version = ["cuda-114"]
cuda-102 = ["cuda-driver-sys-102"]
cuda-114 = ["cuda-driver-sys-114"]
```

### Why start this project

1. My rust project need cuda driver library and need the newest feature for cuda, but the [```cuda-sys```] only has cuda driver 10.2.
2. We can switch cuda version dependence with cargo feature rather than change source code everytime.

[```cuda-sys```]: https://github.com/rust-cuda/cuda-sys
