/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{path::PathBuf, str::FromStr};

use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};
use void_core::filesystem::{
    config::{get_config_folder, get_default_config_scope},
    directories::delete_subdir,
    fs_errors::FsError,
};

/// # Database initialization
/// commonly you don't need to invoke this function, as soon as db starts when app is in loading
/// state.
/// But if you need some config to manage before database starts you could invoke this manually.
/// it will panic if called before config folder initialization
pub async fn init_db(path: Option<String>) -> Surreal<Db> {
    let db_path = if let Some(p) = path {
        PathBuf::from_str(&p).unwrap()
    } else {
        get_config_folder(Some("db"))
    };
    info!("Initialized DB");
    match Surreal::new::<RocksDb>(db_path.clone()).await {
        Ok(db) => db,
        Err(_) => {
            drop_db();
            Surreal::new::<RocksDb>(db_path).await.unwrap()
        }
    }
}

pub fn drop_db() {
    let db_path = get_config_folder(Some("db"));
    info!("database path loaded");
    let mut config = get_default_config_scope();
    if let Err(e) = delete_subdir(&mut config, &db_path) {
        match e {
            FsError::ScopeNotAllowed => {
                error!("error with config scope");
                panic!("error with config scope");
            }
            FsError::GetError(_) => {
                error!("failed to retrieve config folder content");
                panic!("failed to retrieve config folder content");
            }
            FsError::DeleteError(_) => {
                error!("failed to delete config folder content");
                panic!("failed to delete config folder content");
            }
            _ => {
                panic!("unexpected FS error");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::path::PathBuf;
    use void_core::filesystem::directories::get_dir_content;

    #[tokio::test]
    #[serial]
    async fn test_db_creation() {
        init_db(None).await;
        drop_db();
    }

    #[test]
    #[serial]
    fn test_db_drop() {
        drop_db();
        let mut config = get_default_config_scope();
        let db_path = get_config_folder(Some("db"));
        assert_eq!(
            Vec::<PathBuf>::new(),
            get_dir_content(&mut config, &db_path).unwrap()
        );
    }
}
