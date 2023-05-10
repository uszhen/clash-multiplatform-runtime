use std::path::Path;

pub trait PathExt {
    fn to_string_without_extend_length_mark(&self) -> String;
}

impl PathExt for Path {
    fn to_string_without_extend_length_mark(&self) -> String {
        let ret = self.to_string_lossy();

        #[cfg(windows)]
        return ret.strip_prefix("\\\\?\\").unwrap_or(&ret).to_owned();

        #[cfg(not(windows))]
        return ret.to_string();
    }
}
