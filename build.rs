use std::{env, path::PathBuf};

static CUDA_VERSION: &'static str = "cuda-12.1";

macro_rules! cuda_panic {
    () => {
        panic!("CUDA cannot find! You can set the environment 'CUDA_LIBRARY_PATH' or 'CUDA_PATH' to specify it");
    };
}

fn find_cuda_lib(lib_name: &'static str) -> Vec<PathBuf> {
    let split_char;
    let dir_names;

    if cfg!(target_os = "windows") {
        split_char = ";";
        dir_names = vec!["", "x64", "lib\\x64"];
    } else {
        split_char = ":";
        dir_names = vec![
            "",
            "lib64",
            "stubs",
            "lib64/stubs",
            #[cfg(target_arch = "x86_64")]
            "targets/x86_64-linux",
            #[cfg(target_arch = "x86_64")]
            "targets/x86_64-linux/lib",
            #[cfg(target_arch = "aarch64")]
            "targets/aarch64-linux",
            #[cfg(target_arch = "aarch64")]
            "targets/aarch64-linux/lib",
        ];
    }

    let mut candidates: Vec<PathBuf> = env::var("CUDA_LIBRARY_PATH")
        .unwrap_or_default()
        .split(split_char)
        .map(|s| PathBuf::from(s))
        .collect();

    env::var("CUDA_PATH")
        .unwrap_or_default()
        .split(split_char)
        .for_each(|s| candidates.push(PathBuf::from(s)));

    let mut valid_paths = vec![];
    let mut target;

    #[cfg(not(target_os = "windows"))]
    {
        candidates.push(PathBuf::from("/opt/cuda"));
        candidates.push(PathBuf::from("/usr/local/cuda"));
        candidates.push(PathBuf::from("/usr/local/".to_string() + CUDA_VERSION));
    }

    for base in &candidates {
        if base.is_dir() {
            for dir_name in &dir_names {
                target = base.join(dir_name);
                if target.is_dir() && target.join(lib_name).is_file() {
                    valid_paths.push(target);
                }
            }
        }
    }
    valid_paths
}

#[cfg(feature = "cublas")]
fn find_cublas() -> Vec<PathBuf> {
    let lib_name;

    #[cfg(target_os = "windows")]
    {
        lib_name = "cublas.lib";
    }
    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "cuda-static-link")]
        {
            lib_name = "libcublas_static.a";
        }
        #[cfg(not(feature = "cuda-static-link"))]
        {
            lib_name = "libcublas.so";
        }
    }

    let p = find_cuda_lib(lib_name);
    if p.is_empty() {
        cuda_panic!();
    }
    p
}

fn find_cuda() -> Vec<PathBuf> {
    #[cfg(target_os = "windows")]
    let lib_name = "cuda.lib";
    #[cfg(not(target_os = "windows"))]
    let lib_name = "libcuda.so";

    let mut valid_paths = find_cuda_lib(lib_name);
    if valid_paths.is_empty() {
        #[cfg(not(target_os = "windows"))]
        {
            // try find the libcuda.so with gpu driver
            #[cfg(target_arch = "aarch64")]
            {
                let target = PathBuf::from("/usr/lib/aarch64-linux-gnu");
                if target.is_dir() && target.join(lib_name).is_file() {
                    valid_paths.push(target);
                }
            }
            #[cfg(target_arch = "x86_64")]
            {
                let target = PathBuf::from("/usr/lib/x86_64-linux-gnu");
                if target.is_dir() && target.join(lib_name).is_file() {
                    valid_paths.push(target);
                }
            }
        }
        if valid_paths.is_empty() {
            cuda_panic!();
        }
    }

    eprintln!("Found CUDA paths: {:?}", valid_paths);
    valid_paths
}

fn main() {
    for path in find_cuda() {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=dylib=cuda");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "cublas")]
    {
        for path in find_cublas() {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        if cfg!(not(target_os = "windows")) && cfg!(feature = "cuda-static-link") {
            println!(
                "cargo:rustc-flags=-lcudart_static -lcublas_static -lcublasLt_static -lstdc++"
            );
        } else {
            println!("cargo:rustc-flags=-lcudart -lcublas -lcublasLt");
        }
    }
}
