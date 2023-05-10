use std::{error::Error, ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf, ptr::null_mut, slice};

use windows_sys::Win32::{
    Foundation::S_OK,
    UI::Shell::{FOLDERID_LocalAppData, SHGetKnownFolderPath},
};

pub fn current_user_local_directory() -> Result<PathBuf, Box<dyn Error>> {
    let mut ret: *mut u16 = null_mut();

    unsafe {
        if SHGetKnownFolderPath(&FOLDERID_LocalAppData, 0, 0, &mut ret) != S_OK {
            return Err(std::io::Error::last_os_error().into());
        }
    }

    let ret = unsafe {
        let length = (0..).take_while(|&idx| *ret.offset(idx) != 0).count();

        OsString::from_wide(slice::from_raw_parts(ret, length))
    };

    Ok(PathBuf::from(ret))
}
