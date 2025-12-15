#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::unix::ffi::OsStrExt;
use std::ptr::null;
use std::ffi::CString;
use std::path::Path;

mod kappa_c_wrap {
    include!("./hellomod.rs");
}

// #[unsafe(no_mangle)]
// pub extern "C" fn my_hello() {
//     unsafe { hellomod::hello(); }
// }

pub fn a<P: AsRef<Path>>(path: P) {
    unsafe {
        let a = path.as_ref().to_str().unwrap();
        let p = CString::new(a).unwrap();
        kappa_c_wrap::root::a(p.as_ptr());
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

unsafe extern "C" { 
    unsafe fn testcall(v: f32);
}

pub fn test_call(v: f32) {
    unsafe {
        testcall(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn testcall_works() {
        unsafe { 
            testcall(3.9);
        }
    }
    #[test]
    fn hello_works() {
        unsafe { 
            hello();
        }
    }
}
