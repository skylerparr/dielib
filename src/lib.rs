use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn lib_test(methods: HashMap<&str, fn(*mut c_char) -> *mut c_char>) -> *mut c_char {
    println!("Hello from the library!");
    match methods.get("respond") {
        Some(f) => {
            println!("calling the function");
            let c_string = CString::new("print this message please");
            f(CString::into_raw(c_string.unwrap()))
        }
        None => {
            println!("none found");
            CString::new("none found").unwrap().into_raw()
        }
    }
}
