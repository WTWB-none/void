/**
 * Copyright 2025 The VOID Authors. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::{fs, path::Path};

use tauri::Manager;
use tauri_plugin_fs::FsExt;

use super::get_env;
use crate::MAIN_FOLDER_PREFIX;

#[tauri::command]
pub fn get_file(ipath: String) -> Vec<u8> {
    println!("{}", ipath);
    let fpath: &Path = Path::new(&ipath);
    fs::read(fpath).unwrap()
}

#[tauri::command]
pub async fn setup_config_directory(app: tauri::AppHandle) -> Result<(), String> {
    let workdir_conf = MAIN_FOLDER_PREFIX.get().unwrap();
    let plugins_conf = workdir_conf.clone().join("plugins");
    let themes_conf = workdir_conf.join("themes");
    let themes_path = Path::new(&themes_conf);
    let plugins_path = Path::new(&plugins_conf);
    fs::create_dir_all(themes_path).unwrap();
    fs::create_dir_all(plugins_path).unwrap();
    let dest = super::get_env("workdir".to_string(), app.clone())
        .await
        .unwrap()
        + "/profile.png";
    let dest_path = Path::new(&dest);
    let pic = app
        .path()
        .resolve(
            "resources/profile.png",
            tauri::path::BaseDirectory::Resource,
        )
        .unwrap();
    let value = fs::copy(pic, dest_path).unwrap();
    println!("{}", value);
    Ok(())
}

#[tauri::command]
pub async fn allow_scope(app: tauri::AppHandle) -> Result<(), String> {
    let scope = app.fs_scope();
    let workdir = get_env("workdir".to_string(), app.clone()).await.unwrap();
    let _ = scope.allow_directory(workdir, true);
    Ok(())
}
