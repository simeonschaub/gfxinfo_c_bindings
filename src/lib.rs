use gfxinfo::{active_gpu, Gpu};
use std::ffi::CString;
use std::os::raw::c_char;

/// Get a handle to the active GPU
/// The returned handle must be freed by calling gfxinfo_free_gpu
/// Returns null if no GPU is found or an error occurs
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_active_gpu() -> *mut Box<dyn Gpu> {
    match active_gpu() {
        Ok(gpu) => Box::into_raw(Box::new(gpu)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get the vendor name of the GPU
/// The returned string must be freed by calling gfxinfo_free_string
/// Returns null if the handle is invalid
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_get_vendor(handle: *const Box<dyn Gpu>) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gpu = unsafe { &*handle };
    let vendor = gpu.vendor();

    match CString::new(vendor) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get the model name of the GPU
/// The returned string must be freed by calling gfxinfo_free_string
/// Returns null if the handle is invalid
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_get_model(handle: *const Box<dyn Gpu>) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gpu = unsafe { &*handle };
    let model = gpu.model();

    match CString::new(model) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get the family name of the GPU
/// The returned string must be freed by calling gfxinfo_free_string
/// Returns null if the handle is invalid
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_get_family(handle: *const Box<dyn Gpu>) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gpu = unsafe { &*handle };
    let family = gpu.family();

    match CString::new(family) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get the device ID of the GPU
/// The returned string must be freed by calling gfxinfo_free_string
/// Returns null if the handle is invalid
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_get_device_id(handle: *const Box<dyn Gpu>) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gpu = unsafe { &*handle };
    let device_id = gpu.device_id();

    match CString::new(device_id.to_string()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a GPU handle returned by gfxinfo_active_gpu
/// SAFETY: The pointer must have been returned by gfxinfo_active_gpu and not yet freed
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_free_gpu(handle: *mut Box<dyn Gpu>) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Free a string returned by gfxinfo functions
/// SAFETY: The pointer must have been returned by one of the gfxinfo_get_* functions
#[unsafe(no_mangle)]
pub extern "C" fn gfxinfo_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
