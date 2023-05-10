use std::{error::Error, path::PathBuf};

use crate::{metadata::Metadata, win32};

const BASE_DIR_FOSS: &str = "clash-multiplatform-foss";
const BASE_DIR_PREMIUM: &str = "clash-multiplatform";

pub fn current_app_dir() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = std::env::current_exe()?.parent() {
        Ok(path.to_path_buf())
    } else {
        Err("app directory not found".into())
    }
}

pub fn default_base_dir(metadata: &Metadata) -> Result<PathBuf, Box<dyn Error>> {
    let local_dir = win32::dirs::current_user_local_directory()?;

    if metadata.is_premium {
        Ok(local_dir.join(BASE_DIR_PREMIUM))
    } else {
        Ok(local_dir.join(BASE_DIR_FOSS))
    }
}
