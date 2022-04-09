use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

fn call_dynamic(methods: HashMap<&str, fn(*mut c_char) -> *mut c_char>) -> Result<*mut c_char, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("dyno.dll")?;
        let func: libloading::Symbol<unsafe extern fn(HashMap<&str, fn(*mut c_char) -> *mut c_char>) -> *mut c_char> = lib.get(b"lib_test")?;
        Ok(func(methods))
    }
}

fn respond(message: *mut c_char) -> *mut c_char {
    let raw: CString = unsafe { CString::from_raw(message) };
    let to_print = CString::into_string(raw).unwrap();
    println!("responding to your request");
    println!("{}", to_print);
    let ret_val = to_print.clone(); 
    let c_string = CString::new(ret_val);
    CString::into_raw(c_string.unwrap())
}

fn main() {
    println!("Hello, world!");

    let mut methods: HashMap<&str, fn(*mut c_char) -> *mut c_char> = HashMap::new();
    methods.insert("respond", respond);

    let result: CString = unsafe { CString::from_raw(call_dynamic(methods).unwrap()) };
    let to_print = CString::into_string(result).unwrap();
    println!("final print");
    println!("{}", to_print);

    /*
    let mut line = String::new();
    println!("Enter your name :");
    let bytes = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello, {}"), line);
    println!("num bytes, {}", bytes);
    */
}
