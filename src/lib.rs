extern crate ignore;
extern crate libc;
extern crate rustc_serialize;

use ignore::Walk;
use libc::c_char;
use rustc_serialize::json;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn find_files(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();

    let mut res: Vec<String> = Vec::new();

    for entry in Walk::new(r_str) {
        match entry {
            Ok(path) => res.push(path.path().to_string_lossy().into_owned()),
            Err(err) => println!("ERROR: {}", err),
        }
    }

    let json_string = CString::new(json::encode(&res).unwrap()).unwrap();

    json_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_memory(pointer: *mut c_char) {
    unsafe {
        if pointer.is_null() {
            return;
        }
        CString::from_raw(pointer)
    };
}
