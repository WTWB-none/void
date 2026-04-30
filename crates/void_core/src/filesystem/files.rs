/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use rustix::fs::{CWD, RenameFlags, renameat_with};
use std::{
    fs::{File, copy, remove_file, write},
    io::{BufReader, Read},
    path::PathBuf,
};

use void_config::GlobalConfig;

use crate::filesystem::{fs_errors::FsError, secure_fs::check_scope};

/// # simple safe function that reads bytes of specific file
/// returns vector of bytes which represents file
/// requires mutable reference to global config and reference to PathBuf points to target file
/// can return FsError::ScopeNotAllowed or FsError::GetError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///let path = PathBuf::from("/home/transhumanist/.zshrc");
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///let _ = get_file_binary(&mut config, &path).unwrap();
///```
///
pub fn get_file_binary(config: &mut GlobalConfig, path: &PathBuf) -> Result<Vec<u8>, FsError> {
    check_scope(config, path)?;
    if let Err(e) = File::open(path) {
        println!("{}", e);
        error!("file not found!");
        Err(FsError::GetError("file not found!".to_string()))
    } else {
        let buffer = BufReader::new(File::open(path).unwrap());
        info!("get file binaries");
        Ok(buffer
            .bytes()
            .map(|b| {
                b.map_err(|e| {
                    error!("failed to read file bytes");
                    e.to_string()
                })
                .unwrap()
            })
            .collect::<Vec<u8>>())
    }
}

/// # simple safe function that creates file at specific directory
/// returns Ok(()) or Error
/// requires mutable reference to global config, name for file, and reference to PathBuf points to target dir where
/// file will be created
/// can return FsError::ScopeNotAllowed or FsError::CreateError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///let path = PathBuf::from("/home/transhumanist/.zshrc");
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///create_file(&mut config, "test.test".to_string(), &PathBuf::from("/home/transhumanist/tools/"));
///```
pub fn create_file(config: &mut GlobalConfig, name: String, path: &PathBuf) -> Result<(), FsError> {
    check_scope(config, path)?;
    let file_path = path.join(name);
    write(file_path, "").map_err(|e| {
        error!("failed to create file");
        FsError::CreateError(e.to_string())
    })?;
    info!("created file");
    Ok(())
}

/// # simple safe function that renames file
/// returns Ok(()) or Error
/// requires mutable reference to global config, new name for file, and reference to PathBuf points
/// to target file that will be renamed
/// can return FsError::ScopeNotAllowed or FsError::RenameError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///let path = PathBuf::from("/home/transhumanist/.zshrc");
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///rename_file(&mut config, "test.txt".to_string(), &mut PathBuf::from("/home/transhumanist/tools/test.test"));
///```
pub fn rename_file(
    config: &mut GlobalConfig,
    new_name: String,
    file_path: &mut PathBuf,
) -> Result<(), FsError> {
    let fd = CWD;
    check_scope(config, file_path)?;
    let file = file_path.clone();
    file_path.pop();
    let new_file = file_path.join(new_name);
    renameat_with(fd, file, fd, new_file, RenameFlags::NOREPLACE).map_err(|e| {
        error!("failed to rename file");
        FsError::RenameError(e.to_string())
    })?;
    info!("renamed file");
    Ok(())
}

/// # simple safe function that moves file
/// returns Ok(()) or Error
/// requires mutable reference to global config, reference to PathBuf that points to old file, and reference to PathBuf points
/// to target where file will move (including name of file)
/// can return FsError::ScopeNotAllowed or FsError::MoveError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///move_file(&mut config, &PathBuf::from("/home/transhumanist/tools/move.md"), &PathBuf::from("/home/transhumanist/move.md")).unwrap();
///move_file(&mut config, &PathBuf::from("/home/transhumanist/move.md"), &PathBuf::from("/home/transhumanist/tools/move.md")).unwrap();
///```
pub fn move_file(
    config: &mut GlobalConfig,
    old_path: &PathBuf,
    new_path: &PathBuf,
) -> Result<(), FsError> {
    let fd = CWD;
    let mut scope = new_path.clone();
    scope.pop();
    check_scope(config, old_path)?;
    check_scope(config, &scope)?;
    renameat_with(fd, old_path, fd, new_path, RenameFlags::NOREPLACE).map_err(|e| {
        error!("failed to move file");
        FsError::MoveError(e.to_string())
    })?;
    info!("moved file");
    Ok(())
}

/// # simple safe function that copies file
/// returns Ok(()) or Error
/// requires mutable reference to global config, reference to PathBuf that points to old file, and reference to PathBuf points
/// to target where file will be copied (including name of file)
/// can return FsError::ScopeNotAllowed or FsError::CopyError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///copy_file(&mut config, &PathBuf::from("/home/transhumanist/tools/copy.md"), &PathBuf::from("/home/transhumanist/copy.md")).unwrap();
///```
pub fn copy_file(
    config: &mut GlobalConfig,
    file_path: &PathBuf,
    copy_path: &PathBuf,
) -> Result<(), FsError> {
    let mut scope = copy_path.clone();
    scope.pop();
    check_scope(config, file_path)?;
    check_scope(config, &scope)?;
    copy(file_path, copy_path).map_err(|e| {
        error!("failed to copy file");
        FsError::CopyError(e.to_string())
    })?;
    info!("copied file");
    Ok(())
}

/// # simple safe function that deletes file
/// returns Ok(()) or Error
/// requires mutable reference to global config and reference to PathBuf points
/// to target file that will be deleted
/// can return FsError::ScopeNotAllowed or FsError::DeleteError(_s) where _s is the reason why this
/// function returned error
///```ignore
///let mut config = GlobalConfig::default();
///config.change_scope(PathBuf::from("/home/transhumanist/"));
///delete_file(&mut config, &PathBuf::from("/home/transhumanist/tools/copy.md")).unwrap();
///create_file(&mut config, "copy.md".to_string(),
///&PathBuf::from("/home/transhumanist/tools/")).unwrap();
///```
pub fn delete_file(config: &mut GlobalConfig, file_path: &PathBuf) -> Result<(), FsError> {
    check_scope(config, file_path)?;
    remove_file(file_path).map_err(|e| {
        error!("failed to delete file");
        FsError::DeleteError(e.to_string())
    })?;
    info!("deleted file");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_binary_file() {
        let mut config = GlobalConfig::default();
        let path = PathBuf::from("/home/transhumanist/.zshrc");
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        let _ = get_file_binary(&mut config, &path).unwrap();
    }

    #[test]
    #[should_panic(expected = "file not found!")]
    fn test_file_not_exist() {
        let mut config = GlobalConfig::default();
        let path = PathBuf::from("/home/transhumanist/.test-not-exists");
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        let err = get_file_binary(&mut config, &path);
        assert_eq!(Err(FsError::GetError("file not found!".to_string())), err)
    }

    #[test]
    fn test_create_file() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        create_file(
            &mut config,
            "create.test".to_string(),
            &PathBuf::from("/home/transhumanist/tools/"),
        )
        .unwrap();
        delete_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/tools/create.test"),
        )
        .unwrap();
    }

    #[test]
    fn test_rename_file() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        rename_file(
            &mut config,
            "test.test".to_string(),
            &mut PathBuf::from("/home/transhumanist/tools/test.txt"),
        )
        .unwrap();
        rename_file(
            &mut config,
            "test.txt".to_string(),
            &mut PathBuf::from("/home/transhumanist/tools/test.test"),
        )
        .unwrap();
    }

    #[test]
    fn test_move_file() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        move_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/tools/move.md"),
            &PathBuf::from("/home/transhumanist/move.md"),
        )
        .unwrap();
        move_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/move.md"),
            &PathBuf::from("/home/transhumanist/tools/move.md"),
        )
        .unwrap();
    }

    #[test]
    fn test_copy_file() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        copy_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/tools/copy.md"),
            &PathBuf::from("/home/transhumanist/copy.md"),
        )
        .unwrap();
        delete_file(&mut config, &PathBuf::from("/home/transhumanist/copy.md")).unwrap();
    }

    #[test]
    fn test_delete_file() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        delete_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/tools/delete.md"),
        )
        .unwrap();
        create_file(
            &mut config,
            "delete.md".to_string(),
            &PathBuf::from("/home/transhumanist/tools/"),
        )
        .unwrap();
    }
}
