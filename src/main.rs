use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use libloading::Library;


fn main() {
    let mut methods: HashMap<&str, fn(*mut c_char) -> *mut c_char> = HashMap::new();
    methods.insert("respond", respond);

    loop {
        let mut line = String::new();
        println!("Enter your name :");
        let _bytes = std::io::stdin().read_line(&mut line).unwrap();
        let command = line.trim().to_owned();
        //println!("num bytes, {}", bytes);
        if command == "exit" {
            break;
        }
        let methods_to_call = methods.clone();
        let lib: Library = unsafe { libloading::Library::new("dyno.dll") }.unwrap();
        let (lib, response) = call_external(lib, methods_to_call, command);
        let _ = lib.close();

        let result: CString = unsafe { CString::from_raw(response.unwrap()) };
        let to_print = CString::into_string(result).unwrap();
        println!("{}", to_print);
    }
    
}

fn call_external(lib: Library, methods: HashMap<&str, fn(*mut c_char) -> *mut c_char>, command: String) -> (Library, Result<*mut c_char, Box<dyn std::error::Error>>) {
    unsafe {
        let raw_command = CString::new(command).unwrap().into_raw();
        let func: libloading::Symbol<unsafe extern fn(HashMap<&str, fn(*mut c_char) -> *mut c_char>, * mut c_char) -> *mut c_char> = lib.get(b"apply").unwrap();
        let result = Ok(func(methods, raw_command));
        (lib, result)
    }
}

fn respond(message: *mut c_char) -> *mut c_char {
    let raw: CString = unsafe { CString::from_raw(message) };
    let to_print = CString::into_string(raw).unwrap();
    let ret_val = to_print.clone();
    let c_string = CString::new(ret_val);
    CString::into_raw(c_string.unwrap())
}
