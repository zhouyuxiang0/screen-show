use std::{
    error::Error,
    ffi::{CString, NulError},
    ptr,
};
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};

fn main() -> Result<(), NulError> {
    println!("Hello, world!");
    let lp_text = CString::new("hello world")?;
    let lp_caption = CString::new("test")?;
    unsafe {
        MessageBoxA(
            ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    Ok(())
}
