use crate::internal::*;
use crate::*;

use sdl2_sys::SDL_GetPrefPath;
use tracing::error;

use std::io::ErrorKind;
use std::{env, fs};

#[repr(C)]
pub struct Directory {
    pub iterator: fs::ReadDir,
}

#[no_mangle]
pub unsafe extern "C" fn Directory_Open(path: *const libc::c_char) -> *mut Directory {
    match fs::read_dir(path.as_str()) {
        Ok(dir) => {
            let this = MemNew!(Directory);
            (*this).iterator = dir;
            this
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn Directory_Close(this: *mut Directory) {
    MemDelete!(this);
}

#[no_mangle]
pub extern "C" fn Directory_GetNext(this: &mut Directory) -> *const libc::c_char {
    match this.iterator.next() {
        Some(Ok(dir)) => dir
            .file_name()
            .to_str()
            .map(|s| s.to_string())
            .map(|s| static_string!(s))
            .unwrap_or(std::ptr::null()),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn Directory_Change(cwd: *const libc::c_char) -> bool {
    env::set_current_dir(cwd.as_str()).is_ok()
}

// This will create the directory if it doesn't exist, or do nothing if it exists already.
#[no_mangle]
pub extern "C" fn Directory_Create(path: *const libc::c_char) -> bool {
    match fs::create_dir(path.as_str()) {
        Ok(()) => true,
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => true,
            _ => {
                error!("Directory_Create: Failed to create directory: {err}");

                false
            }
        },
    }
}

#[no_mangle]
pub extern "C" fn Directory_GetCurrent() -> *const libc::c_char {
    match env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(path_str) => static_string!(path_str),
            None => std::ptr::null(),
        },
        Err(_) => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn Directory_GetPrefPath(
    org: *const libc::c_char,
    app: *const libc::c_char,
) -> *const libc::c_char {
    unsafe { SDL_GetPrefPath(org, app) }
}

#[no_mangle]
pub extern "C" fn Directory_Remove(path: *const libc::c_char) -> bool {
    fs::remove_dir(path.as_str()).is_ok()
}
