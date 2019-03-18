#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
//use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use libc::{c_void, c_char};

#[no_mangle]
extern fn basque_cmd(ctx: *mut sqlite3_context,
                     argc: ::std::os::raw::c_int,
                     argv: *mut *mut sqlite3_value) {
    let routines = unsafe { (*GLOBAL_ROUTINES) };
    let state_ptr = unsafe { (routines.user_data.unwrap())(ctx) };
    println!("basque_cmd called with {} args and context: {:p}", argc, state_ptr);
    //let internal_state: Box<Box<InternalState>> = unsafe { Box::from_raw(state_ptr as *mut Box<InternalState>) };
    let magic = unsafe { (*(state_ptr as *mut Box<InternalState>)).magic };
    // Was hoping for 69. Get "1" or other random numbers,
    // so I'm grabbing the wrong memory somehow.
    println!("magic value from context is: {}", magic);
    unsafe { (routines.result_int64.unwrap())(ctx, magic) };
}

#[no_mangle]
extern fn basque_destroy(state_ptr: *mut c_void) {
    println!("finalizing");
    unsafe { Box::from_raw(state_ptr as *mut Box<InternalState>) };
}


struct InternalState {
    magic: i64
}

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

// rot13 example extension: https://www.sqlite.org/src/file/ext/misc/rot13.c

static mut GLOBAL_ROUTINES: *const sqlite3_api_routines = 0 as *const sqlite3_api_routines;

#[no_mangle]
pub unsafe extern fn sqlite3_basque_init(db: *mut sqlite3, err: *mut *const c_char, routines: *const sqlite3_api_routines) -> u32 {
    let msg = CString::new("Not yet implemented!").expect("Failed to allocate error");
    let msg_ptr: *const c_char = ((*routines).mprintf.unwrap())(msg.as_ptr());
    std::mem::forget(msg);
    *err = msg_ptr;

    GLOBAL_ROUTINES = routines;
    // We need to keep a reference to `routines` to call sqlite's API later (we're not
    // allowed to link to it, since we need the version that loaded us as a library.)
    // SQLite recommends you do this using a macro called SQLITE_EXTENSION_INIT2,
    // which stuffs the pointer into a static. I'd rather keep it inside our
    // callback context so we can avoid a global.
    let internal_state = Box::new(Box::new(InternalState { magic: 69 }));
    let internal_state_ptr = Box::into_raw(internal_state);
    println!("raw context pointer: {:p}", internal_state_ptr);

    let fn_name = CString::new("basque_cmd").expect("Failed to allocate name");

    ((*routines).create_function_v2.unwrap())(db, fn_name.as_ptr(), 1, SQLITE_UTF8 as i32,
                                              internal_state_ptr as *mut c_void,
                                              Some(basque_cmd), None, None, Some(basque_destroy));

    return SQLITE_OK;
}
