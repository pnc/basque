#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
use std::process::Command;
//use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
use libc::{c_void, c_char};

struct SqliteRoutines {
    routines: *const sqlite3_api_routines,
}

impl SqliteRoutines {
    fn user_data(&self, ctx: *mut sqlite3_context) -> *mut c_void {
        let internal = unsafe { (*self.routines).user_data.unwrap() };
        unsafe { internal(ctx) }
    }

    fn result_text(&self, ctx: *mut sqlite3_context,
                   buffer: *const ::std::os::raw::c_char,
                   length: ::std::os::raw::c_int,
                   free: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>) {
        let internal = unsafe { (*self.routines).result_text.unwrap() };
        unsafe { internal(ctx, buffer, length, free) }
    }
}

#[no_mangle]
extern fn basque_cmd(ctx: *mut sqlite3_context,
                     argc: ::std::os::raw::c_int,
                     argv: *mut *mut sqlite3_value) {
    let routines = unsafe { GLOBAL_ROUTINES.as_ref().unwrap() };
    let state_ptr = routines.user_data(ctx);
    println!("basque_cmd called with {} args and context: {:p}", argc, state_ptr);
    let magic = unsafe { (*(state_ptr as *mut Box<InternalState>)).magic };
    // Was hoping for 69. Get "1" or other random numbers,
    // so I'm grabbing the wrong memory somehow.
    println!("magic value from context is: {}", magic);
    //let result = "Okay! 🐬";

    let result = Command::new("ps")
        .arg("x")
        .arg("-o")
        .arg("command")
        .output()
        .expect("failed to execute process").stdout;

    let result_str = String::from_utf8(result).unwrap();
    println!("result: {}", result_str);

    // TODO: set destructor context and free result properly
    routines.result_text(ctx, result_str.as_ptr() as *const i8, result_str.len() as i32, None);
    std::mem::forget(result_str);
}

#[no_mangle]
extern fn basque_destroy(state_ptr: *mut c_void) {
    // from_raw takes ownership back and drops the memory
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

//static mut GLOBAL_ROUTINES: *const sqlite3_api_routines = 0 as *const sqlite3_api_routines;
static mut GLOBAL_ROUTINES: Option<SqliteRoutines> = None;

#[no_mangle]
pub unsafe extern fn sqlite3_basque_init(db: *mut sqlite3, err: *mut *const c_char, routines: *const sqlite3_api_routines) -> u32 {
    let msg = CString::new("Not yet implemented!").expect("Failed to allocate error");
    let msg_ptr: *const c_char = ((*routines).mprintf.unwrap())(msg.as_ptr());
    std::mem::forget(msg);
    *err = msg_ptr;

    // We need to keep a reference to `routines` to call sqlite's API later (we're not
    // allowed to link to it, since we need the version that loaded us as a library.)
    // SQLite recommends you do this using a macro called SQLITE_EXTENSION_INIT2,
    // which stuffs the pointer into a static. We wrap it in a struct to abstract
    // the unsafe calls it has to make.
    //(*GLOBAL_ROUTINES.lock().unwrap()) = Some(SqliteRoutines { routines: routines });
    GLOBAL_ROUTINES = Some(SqliteRoutines { routines: routines });

    let internal_state = Box::new(Box::new(InternalState { magic: 69 }));
    let internal_state_ptr = Box::into_raw(internal_state);
    println!("raw context pointer: {:p}", internal_state_ptr);

    let fn_name = CString::new("basque_cmd").expect("Failed to allocate name");

    ((*routines).create_function_v2.unwrap())(db, fn_name.as_ptr(), 1, SQLITE_UTF8 as i32,
                                              internal_state_ptr as *mut c_void,
                                              Some(basque_cmd), None, None, Some(basque_destroy));

    return SQLITE_OK;
}
