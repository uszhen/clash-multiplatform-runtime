use std::iter::once;

pub trait Win32Strings {
    fn to_win32_utf16(&self) -> Vec<u16>;
}

impl Win32Strings for String {
    fn to_win32_utf16(&self) -> Vec<u16> {
        self.encode_utf16().chain(once(0)).collect::<Vec<u16>>()
    }
}

impl Win32Strings for &str {
    fn to_win32_utf16(&self) -> Vec<u16> {
        self.encode_utf16().chain(once(0)).collect::<Vec<u16>>()
    }
}
