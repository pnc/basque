use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern fn sqlite3_basque_init(db: u64, err: *mut *const c_char, routines: u64) -> i32 {
    let msg = CString::new("Not yet implemented!").expect("Failed to allocate error");
    let msg_ptr = msg.as_ptr();
    std::mem::forget(msg);
    *err = msg_ptr;
    return 1;
}
