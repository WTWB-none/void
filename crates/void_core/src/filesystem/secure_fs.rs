use std::{fs::canonicalize, path::PathBuf};
use void_entities::config::GlobalConfig;

use crate::filesystem::fs_errors::FsError;

/// # checks if user allowed to access this directory.
/// if scope not set returns FsError::ScopeNotAllowed
/// also returns FsError::ScopeNotAllowed if directory/file not exist. If you encounter this error create dirs first
/// and then check scope
pub fn check_scope(config: &mut GlobalConfig, path: &PathBuf) -> Result<(), FsError> {
    match config.get_scope() {
        Some(s) => {
            if canonicalize(path).is_err() {
                return Err(FsError::ScopeNotAllowed);
            }
            if path
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(s.canonicalize().unwrap().to_str().unwrap())
            {
                return Ok(());
            }
            Err(FsError::ScopeNotAllowed)
        }
        None => Err(FsError::ScopeNotAllowed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allowed_scope() {
        let mut conf = GlobalConfig::default();
        let path = dirs_next::home_dir().unwrap();
        conf.change_scope(path.clone());
        assert_eq!(Ok(()), check_scope(&mut conf, &path.join("Загрузки")))
    }

    #[test]
    fn not_allowed_scope() {
        let mut conf = GlobalConfig::default();
        let path = dirs_next::home_dir().unwrap();
        conf.change_scope(path.clone());
        assert_eq!(
            Err(FsError::ScopeNotAllowed),
            check_scope(&mut conf, &PathBuf::from("/usr/bin"))
        )
    }
}
