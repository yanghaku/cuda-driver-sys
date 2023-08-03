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
cuda-driver-sys-118 = { package = "cuda-driver-sys", version = "0.3", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-118", optional = true }
cuda-driver-sys-122 = { package = "cuda-driver-sys", version = "0.3", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-122", optional = true }

[features]
default-cuda-version = ["cuda-122"]
cuda-102 = ["cuda-driver-sys-102"]
cuda-118 = ["cuda-driver-sys-118"]
cuda-122 = ["cuda-driver-sys-122"]
```

### Why start this project

1. My rust project need cuda driver library and need the newest feature for cuda, but the [```cuda-sys```] only has cuda driver 10.2.
2. We can switch cuda version dependence with cargo feature rather than change source code everytime.

[```cuda-sys```]: https://github.com/rust-cuda/cuda-sys
