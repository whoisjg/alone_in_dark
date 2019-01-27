use std::ffi::OsString;
use std::path::PathBuf;

pub fn validate<'a>(
    path: &PathBuf,
    is_dir: bool,
    extensions: impl Into<Option<&'a Vec<OsString>>>,
) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("{} does not exist", path.display()));
    }
    Ok(())
}
