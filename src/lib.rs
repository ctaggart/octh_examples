#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate octh;

use std::ffi::CString;

// https://github.com/ctaggart/octh/issues/18
// https://thefullsnack.com/en/string-ffi-rust.html

#[no_mangle]
pub unsafe extern "C"  fn Ghelloworld (shl: *const octh::root::octave::dynamic_library, relative: bool) -> *mut octh::root::octave_dld_function {

    let fname = CString::new("Fhelloworld").unwrap();
    let pfname = fname.as_ptr() as octh::root::octave_builtin_fcn;
    std::mem::forget(pfname);

    let name = CString::new("helloworld").unwrap();
    let pname = name.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pname);

    let doc = CString::new("Hello World Help String").unwrap();
    let pdoc = doc.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pdoc);

//    let fcn = octh::root::octave_dld_function_create(pfname as u64, shl, pname as u64, pdoc as u64);
//    octh::root::octave_dld_function::create()
    let fcn = octh::root::octave_dld_function_create(pfname, shl, pname, pdoc);

//    if relative {
//        fcn.mark_relative();
//    }
    return fcn;
}    

 pub unsafe extern "C"  fn Fhelloworld (args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
     let out = octh::root::octave_value_list_create();
     return out;
 }