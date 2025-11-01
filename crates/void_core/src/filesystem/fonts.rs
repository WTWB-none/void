use std::{
    ffi::OsStr,
    fs::{read_dir, rename},
    path::PathBuf,
};

use crate::filesystem::{config::get_config_folder, fs_errors::FsError};

/// # basic utility that returns Vec of PathBuf with all fonts in th e fonts directory
/// if fonts dir not present in config folder creates new and returns empty vec![]
/// ```ignore
/// let fonts = get_fonts();
/// assert_eq!(Vec::<PathBuf>::new(), fonts);
/// ```
pub fn get_fonts() -> Vec<PathBuf> {
    let fonts_folder = get_config_folder(Some("fonts"));
    let entries = read_dir(fonts_folder).unwrap();
    entries
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.metadata().unwrap().is_file())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("ttf")))
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>()
}

/// # function that allows adding user fonts to VOID. I think that it wouldn't panic, but if it
/// would, please report an issue to our repo
/// returns error if not font file path provided
pub fn add_font(font_path: PathBuf) -> Result<(), FsError> {
    let fonts_folder = get_config_folder(Some("fonts"));
    match font_path.extension() {
        Some(s) => {
            if s.to_str() != Some("ttf") {
                error!("invalid font provided");
                return Err(FsError::MoveError("not valid file provided".to_string()));
            }
        }
        None => {
            error!("invalid font provided");
            return Err(FsError::MoveError("not valid file provided".to_string()));
        }
    }
    let font_name = font_path.clone().file_name().unwrap().to_owned();
    rename(font_path, fonts_folder.join(font_name)).map_err(|e| {
        error!("failed to move font");
        FsError::MoveError(e.to_string())
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fonts_list() {
        let fonts = get_fonts();
        assert_eq!(Vec::<PathBuf>::new(), fonts);
    }
}
