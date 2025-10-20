use crate::filesystem::{fs_errors::FsError, secure_fs::check_scope};
use dircpy::copy_dir_advanced;
use rustix::fs::{RenameFlags, renameat_with};
use std::{
    fs::{create_dir, read_dir, remove_dir_all},
    path::PathBuf,
};
use void_entities::GlobalConfig;

/// # simple safe function that gets content of specific directory
/// returns vector of PathBuf which represents all items in chosen directory
/// requires mutable reference to global config and reference to PathBuf points to target dir
/// can return FsError::ScopeNotAllowed or FsError::GetError(_s) where _s is the reason why this
/// function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::get_dir_content;
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/tools/"));
/// let vec = get_dir_content(&mut config, &PathBuf::from("/home/transhumanist/tools/")).unwrap();
/// ```
pub fn get_dir_content(config: &mut GlobalConfig, dir: &PathBuf) -> Result<Vec<PathBuf>, FsError> {
    check_scope(config, dir)?;
    let entries = read_dir(dir).map_err(|e| FsError::GetError(e.to_string()))?;
    let vec_entries = entries
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();
    Ok(vec_entries)
}

/// # simple safe function that creates subdir in workspace
/// returns () if creation was successfull or Error
/// requires mutable reference to global config and reference to PathBuf points to target dir that
/// not exists
/// Error could be FsError::ScopeNotAllowed or FsError::GetError(_s) where _s is the reason why
/// this function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::{create_subdir, delete_subdir};
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/tools/"));
/// create_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/test/")).unwrap();
/// delete_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/test/")).unwrap();
/// ```
pub fn create_subdir(config: &mut GlobalConfig, path: &PathBuf) -> Result<(), FsError> {
    let mut check = path.clone();
    check.pop();
    check_scope(config, &check)?;
    create_dir(path).map_err(|e| FsError::CreateError(e.to_string()))?;
    Ok(())
}

/// # simple safe function that moves subdir across workspace
/// returns () if move was successfull or Error
/// requires mutable reference to global config, reference to PathBuf that points to directory that
/// will be moved and reference to PathBuf that points to new path (include name of dir that
/// moves).
/// Error could be FsError::ScopeNotAllowed and FsError::MoveError(_s) where _s is the reason why
/// this function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::move_subdir;
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/"));
/// move_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/"), &PathBuf::from("/home/transhumanist/projects/tools")).unwrap();
/// move_subdir(&mut config, &PathBuf::from("/home/transhumanist/projects/tools/"), &PathBuf::from("/home/transhumanist/tools")).unwrap();
/// ```
pub fn move_subdir(
    config: &mut GlobalConfig,
    old_path: &PathBuf,
    new_path: &PathBuf,
) -> Result<(), FsError> {
    let fd = rustix::fs::CWD;
    let mut check = new_path.clone();
    check.pop();
    check_scope(config, old_path)?;
    check_scope(config, &check)?;
    renameat_with(fd, old_path, fd, new_path, RenameFlags::NOREPLACE)
        .map_err(|e| FsError::MoveError(e.to_string()))?;
    Ok(())
}

/// # simple safe function that copies subdir into another destination
/// returns () if copy was successfull or Error
/// requires mutable reference to global config, reference to PathBuf that points to directory that
/// will be copied and reference to PathBuf that points to new path (include name of dir that
/// copying).
/// Error could be FsError::ScopeNotAllowed and FsError::CopyError(_s) where _s is the reason why
/// this function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::{copy_subdir, delete_subdir};
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/"));
/// copy_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/"), &PathBuf::from("/home/transhumanist/projects/tools")).unwrap();
/// delete_subdir(&mut config, &PathBuf::from("/home/transhumanist/projects/tools/")).unwrap();
/// ```
pub fn copy_subdir(
    config: &mut GlobalConfig,
    target_path: &PathBuf,
    dest_path: &PathBuf,
) -> Result<(), FsError> {
    let mut check = dest_path.clone();
    check.pop();
    check_scope(config, target_path)?;
    check_scope(config, &check)?;
    copy_dir_advanced(target_path, dest_path, false, false, false, vec![], vec![])
        .map_err(|e| FsError::CopyError(e.to_string()))?;
    Ok(())
}

/// # simple safe function that renames workspace directory
/// returns () if rename was successfull or Error
/// requires mutable reference to global config, reference to PathBuf that points to directory that
/// will be renamed and new name
/// Error could be FsError::ScopeNotAllowed and FsError::RenameError(_s) where _s is the reason why
/// this function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::rename_subdir;
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/"));
/// rename_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/rename_dir"),
/// String::from("renamed_dir")).unwrap();
/// rename_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/renamed_dir"),
/// String::from("rename_dir")).unwrap();
///
/// ```
pub fn rename_subdir(
    config: &mut GlobalConfig,
    old_path: &PathBuf,
    new_name: String,
) -> Result<(), FsError> {
    let fd = rustix::fs::CWD;
    check_scope(config, old_path)?;
    let mut new_path = old_path.clone();
    new_path.pop();
    new_path = new_path.join(new_name);
    renameat_with(fd, old_path, fd, new_path, RenameFlags::NOREPLACE)
        .map_err(|e| FsError::RenameError(e.to_string()))?;
    Ok(())
}

/// # simple safe function that deletes workspace subdirectory
/// returns () if deletion was successfull or Error
/// requires mutable reference to global config, reference to PathBuf that points to directory that
/// will be deleted
/// Error could be FsError::ScopeNotAllowed and FsError::DeleteError(_s) where _s is the reason why
/// this function returned error
/// ```
/// use void_entities::GlobalConfig;
/// use void_core::filesystem::directories::{create_subdir, delete_subdir};
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/"));
/// create_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/to_delete/")).unwrap();
/// delete_subdir(&mut config, &PathBuf::from("/home/transhumanist/tools/to_delete/")).unwrap();
/// ```
pub fn delete_subdir(config: &mut GlobalConfig, path: &PathBuf) -> Result<(), FsError> {
    check_scope(config, path)?;
    remove_dir_all(path).map_err(|e| FsError::DeleteError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use void_entities::GlobalConfig;

    #[test]
    fn scope_error() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/tools/"));
        let vec = get_dir_content(&mut config, &PathBuf::from("/home/transhumanist/"));
        assert_eq!(FsError::ScopeNotAllowed, vec.err().unwrap());
    }
}
