#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
include!("cuda.rs");

#[cfg(feature = "cublas")]
pub mod cublas;

#[cfg(test)]
mod tests {
    use super::*;

    // not a test for get driver version, but a test for linking
    #[test]
    fn link_test() {
        let mut version: i32 = 0;
        let result = unsafe { cuDriverGetVersion(&mut version as *mut i32) };
        match result {
            CUresult::CUDA_SUCCESS => {
                println!("Deriver Version = {:?}", version);
            }
            _ => {
                println!("Cannot get driver version");
            }
        }
    }

    #[test]
    #[cfg(feature = "cublas")]
    fn link_cublas_test() {
        use cublas::*;

        let mut handle = std::ptr::null_mut();
        let result = unsafe { cublasCreate_v2(&mut handle as *mut _) };
        match result {
            cublasStatus_t::CUBLAS_STATUS_SUCCESS => {
                println!("Succeed to init CUBLAS");
            }
            cublasStatus_t::CUBLAS_STATUS_NOT_INITIALIZED => {
                println!("GPU not found");
            }
            _ => {
                panic!("Failed to init CUBLAS");
            }
        }
    }
}
