use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;

use crate::error::FstError;
use crate::ffi;

/// A morphological analyzer that uses libfoma via FFI for better performance.
pub struct FomaFfiAnalyzer {
    fsm: *mut ffi::Fsm,
    handle: *mut ffi::ApplyHandle,
}

// FomaFfiAnalyzer is not thread-safe by default because of the internal handle
// but we can wrap it or manage it. libfoma's apply_handle is stateful.
unsafe impl Send for FomaFfiAnalyzer {}

impl FomaFfiAnalyzer {
    /// Create a new FomaFfiAnalyzer from an FST file.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FstError> {
        let path_str = path.as_ref().to_str().ok_or_else(|| {
            FstError::FfiError("Invalid path encoding".to_string())
        })?;
        let c_path = CString::new(path_str).map_err(|e| {
            FstError::FfiError(format!("Invalid path: {}", e))
        })?;

        unsafe {
            let fsm = ffi::fsm_read_binary_file(c_path.as_ptr());
            if fsm.is_null() {
                return Err(FstError::FstFileNotFound(path.as_ref().to_path_buf()));
            }

            let handle = ffi::apply_init(fsm);
            if handle.is_null() {
                ffi::fsm_destroy(fsm);
                return Err(FstError::FfiError("Failed to initialize foma apply handle".to_string()));
            }

            Ok(Self { fsm, handle })
        }
    }

    /// Analyze a word using the FST (downward lookup: surface -> analysis).
    pub fn analyze(&self, word: &str) -> Result<Vec<String>, FstError> {
        let c_word = CString::new(word).map_err(|e| {
            FstError::FfiError(format!("Invalid word: {}", e))
        })?;

        let mut results = Vec::new();
        unsafe {
            let res_ptr = ffi::apply_down(self.handle, c_word.as_ptr());
            if !res_ptr.is_null() {
                let res = CStr::from_ptr(res_ptr).to_string_lossy().into_owned();
                results.push(res);

                // Get subsequent results
                loop {
                    let next_ptr = ffi::apply_down(self.handle, ptr::null());
                    if next_ptr.is_null() {
                        break;
                    }
                    let res = CStr::from_ptr(next_ptr).to_string_lossy().into_owned();
                    results.push(res);
                }
            } else {
                // Fallback: try apply_up if apply_down failed
                let res_up_ptr = ffi::apply_up(self.handle, c_word.as_ptr());
                if !res_up_ptr.is_null() {
                    let res = CStr::from_ptr(res_up_ptr).to_string_lossy().into_owned();
                    results.push(res);
                    
                    loop {
                        let next_ptr = ffi::apply_up(self.handle, ptr::null());
                        if next_ptr.is_null() {
                            break;
                        }
                        let res = CStr::from_ptr(next_ptr).to_string_lossy().into_owned();
                        results.push(res);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Generate forms from an analysis (upward lookup: analysis -> surface).
    pub fn generate(&self, analysis: &str) -> Result<Vec<String>, FstError> {
        let c_analysis = CString::new(analysis).map_err(|e| {
            FstError::FfiError(format!("Invalid analysis string: {}", e))
        })?;

        let mut results = Vec::new();
        unsafe {
            let res_ptr = ffi::apply_up(self.handle, c_analysis.as_ptr());
            if !res_ptr.is_null() {
                let res = CStr::from_ptr(res_ptr).to_string_lossy().into_owned();
                results.push(res);

                // Get subsequent results
                loop {
                    let next_ptr = ffi::apply_up(self.handle, ptr::null());
                    if next_ptr.is_null() {
                        break;
                    }
                    let res = CStr::from_ptr(next_ptr).to_string_lossy().into_owned();
                    results.push(res);
                }
            }
        }

        Ok(results)
    }
}

impl Drop for FomaFfiAnalyzer {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                ffi::apply_clear(self.handle);
            }
            if !self.fsm.is_null() {
                ffi::fsm_destroy(self.fsm);
            }
        }
    }
}
