use std::{error::Error, io, path::Path, ptr::null_mut};

use cstr::cstr;
use windows_sys::Win32::{
    Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE, SetHandleInformation, TRUE},
    Storage::FileSystem::{
        CREATE_ALWAYS, CreateFileA, CreateFileW, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    },
    System::Console::{AllocConsole, GetConsoleWindow, SetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    UI::WindowsAndMessaging::{ShowWindow, SW_HIDE},
};

use crate::win32::strings::Win32Strings;

pub fn redirect_standard_output_to_file(path: &Path) -> Result<(), Box<dyn Error>> {
    unsafe {
        if AllocConsole() == TRUE {
            let console = GetConsoleWindow();

            ShowWindow(console, SW_HIDE);
        }
    }

    let path = path.to_str().unwrap().to_win32_utf16();

    unsafe {
        let handle = CreateFileW(
            path.as_ptr(),
            GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_DELETE,
            null_mut(),
            CREATE_ALWAYS,
            0,
            0,
        );
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error().into());
        }

        SetHandleInformation(handle, HANDLE_FLAG_INHERIT, TRUE as u32);

        SetStdHandle(STD_OUTPUT_HANDLE, handle);
        SetStdHandle(STD_ERROR_HANDLE, handle);
    }

    unsafe {
        let nul = CreateFileA(
            cstr!("nul:").as_ptr().cast(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            null_mut(),
            OPEN_EXISTING,
            0,
            0,
        );

        SetHandleInformation(nul, HANDLE_FLAG_INHERIT, TRUE as u32);

        SetStdHandle(STD_INPUT_HANDLE, nul);
    }

    Ok(())
}
