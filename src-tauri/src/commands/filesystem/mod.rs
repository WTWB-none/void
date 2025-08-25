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
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

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
    let fonts_conf = workdir_conf.join("fonts");
    let themes_path = Path::new(&themes_conf);
    let plugins_path = Path::new(&plugins_conf);
    let fonts_path = Path::new(&fonts_conf);
    fs::create_dir_all(themes_path).unwrap();
    fs::create_dir_all(plugins_path).unwrap();
    fs::create_dir_all(fonts_path).unwrap();
    let dest = MAIN_FOLDER_PREFIX.get().unwrap().join("profile.png");
    let pic = app
        .path()
        .resolve(
            "resources/profile.png",
            tauri::path::BaseDirectory::Resource,
        )
        .unwrap();
    let _ = fs::copy(pic, dest).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_config_directory() -> String {
    MAIN_FOLDER_PREFIX
        .get()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[tauri::command]
pub async fn allow_scope(app: tauri::AppHandle) {
    let scope = app.fs_scope();
    let workdir = get_env("workdir".to_string(), app.clone()).await.unwrap();
    let _ = scope.allow_directory(workdir, true);
}

#[tauri::command]
pub fn copy_font(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let filename = path.split('/').last();
    #[cfg(target_os = "linux")]
    let filename = path.split("\\").last();
    #[cfg(target_os = "windows")]
    let filename = path.split('/').last();

    let user_path = std::path::PathBuf::from_str(&path).unwrap();
    let fonts_folder = MAIN_FOLDER_PREFIX
        .get()
        .unwrap()
        .join("fonts")
        .join(filename.unwrap());
    std::fs::copy(user_path, fonts_folder).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_all_user_fonts() -> Result<Vec<String>, String> {
    let paths = std::fs::read_dir(MAIN_FOLDER_PREFIX.get().unwrap().join("fonts"))
        .map_err(|e| e.to_string())?;
    let paths = paths.map(|f| f.unwrap().path());
    Ok(paths
        .map(|f| f.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<String>>())
}
