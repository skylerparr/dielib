use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn apply(methods: HashMap<&str, fn(*mut c_char) -> *mut c_char>, fun_name: *mut c_char) -> *mut c_char {
    println!("Hello from the library!");
    let raw: CString = unsafe { CString::from_raw(fun_name) };
    let command = CString::into_string(raw).unwrap();
    let command_str: &str = &command[..];
    match methods.get(command_str) {
        Some(f) => {
            println!("calling the function...");
            let c_string = CString::new("print this message please");
            f(CString::into_raw(c_string.unwrap()))
        }
        None => {
            let response = format!("command {} not found", command);
            println!("{}", response);
            CString::new(response).unwrap().into_raw()
        }
    }
}
