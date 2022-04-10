use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn apply(methods: HashMap<&str, fn(*mut c_char) -> *mut c_char>, fun_name: *mut c_char) -> *mut c_char {
    let raw: CString = unsafe { CString::from_raw(fun_name) };
    let command = CString::into_string(raw).unwrap();
    let command_str: &str = &command[..];
    match methods.get(command_str) {
        Some(f) => {
            let c_string = CString::new("This is a fun function");
            f(CString::into_raw(c_string.unwrap()))
        }
        None => {
            let response = format!("command {} not found", command);
            println!("{}", response);
            CString::new(response).unwrap().into_raw()
        }
    }
}
