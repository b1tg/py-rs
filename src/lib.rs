use std::ffi::CString;
use std::mem::forget;
use std::os::raw::c_char;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern "C" fn lib_test() {
    println!("message from py-rs");
}

#[no_mangle]
pub extern "C" fn lib_get_str() -> *const c_char {
    let c_string = CString::new("foo").expect("CString::new failed");
    let ptr = c_string.into_raw();
    return ptr;
}
#[no_mangle]
pub extern "C" fn lib_scan(input: *const c_char) -> *const c_char {
    unsafe {
        let input = CString::from_raw(input as *mut c_char);
        let input = input.into_string().unwrap_or("bad path".to_string());
        let res = format!("scan {} over...", &input);
        // important!
        forget(input);

        let c_string = CString::new(res).expect("CString::new failed");
        let ptr = c_string.into_raw();
        return ptr;
    }
}
