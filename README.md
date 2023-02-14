# cuda-driver-sys

Rust binding to CUDA driver library (```libcuda.so```)

This project focuses only ```cuda driver api```, the full cuda-sys can see [```cuda-sys```]

### Usage

1. just one version
   such as cuda version 10.2

```toml
[dependencies]
cuda-driver-sys = { version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-102" }
```

2. multi versions with cargo feature

```toml
[dependencies]
cuda-driver-sys-102 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-102", optional = true }
cuda-driver-sys-115 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-115", optional = true }
cuda-driver-sys-117 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-117", optional = true }
cuda-driver-sys-118 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-118", optional = true }
cuda-driver-sys-120 = { package = "cuda-driver-sys", version = "0.3.1", git = "https://github.com/yanghaku/cuda-driver-sys", branch = "cuda-120", optional = true }

[features]
default-cuda-version = ["cuda-120"]
cuda-102 = ["cuda-driver-sys-102"]
cuda-115 = ["cuda-driver-sys-115"]
cuda-117 = ["cuda-driver-sys-117"]
cuda-118 = ["cuda-driver-sys-118"]
cuda-120 = ["cuda-driver-sys-120"]
```

### Why start this project

1. My rust project need cuda driver library and need the newest feature for cuda, but the [```cuda-sys```] only has cuda
   driver 10.2.
2. We can switch cuda version dependence with cargo feature rather than change source code everytime.

### version info

* branch cuda-102: copy src/*.rs from [```cuda-sys/cuda-driver-sys```], and rewrite build.rs
* branch cuda-115+: use ```bindgen``` to generate

[```cuda-sys```]: https://github.com/rust-cuda/cuda-sys
