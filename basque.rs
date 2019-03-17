#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
//use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use libc::{c_int, c_void, c_char};

// https://www.sqlite.org/loadext.html
// int sqlite3_extension_init(
//   sqlite3 *db, 
//   char **pzErrMsg, 
//   const sqlite3_api_routines *pApi
// ){
//   int rc = SQLITE_OK;
//   SQLITE_EXTENSION_INIT2(pApi);
//   /* Insert here calls to
//   **     sqlite3_create_function_v2(),
//   **     sqlite3_create_collation_v2(),
//   **     sqlite3_create_module_v2(), and/or
//   **     sqlite3_vfs_register()
//   ** to register the new features that your extension adds.
//   */
//   return rc;
// }

#[no_mangle]
pub unsafe extern fn sqlite3_basque_init(db: *const c_void, err: *mut *const c_char, routines: *const sqlite3_api_routines) -> c_int {
    let msg = CString::new("Not yet implemented!").expect("Failed to allocate error");
    //let msg_ptr = msg.as_ptr();
    println!("String is in {:p}", (*routines).mprintf.unwrap());
    let msg_ptr: *const c_char = ((*routines).mprintf.unwrap())(msg.as_ptr());
    println!("sqlite fmt is at {:p}", msg_ptr);
    std::mem::forget(msg);
    *err = msg_ptr;
    return 1;
}
