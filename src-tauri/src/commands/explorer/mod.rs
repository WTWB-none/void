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
use tauri::Emitter;

use super::get_env;

#[tauri::command]
pub async fn get_directory_content(dirname: String, app: tauri::AppHandle) -> Vec<String> {
    let workdir = super::get_env("workdir".to_string(), app.clone())
        .await
        .unwrap();
    let paths = std::fs::read_dir(workdir.clone() + "/" + dirname.as_str()).unwrap();
    let dirs: Vec<String> = paths
        .map(|e| e.unwrap().path().to_str().unwrap().to_string())
        .map(|d| d.replace((workdir.clone() + "/").as_str(), ""))
        .collect();
    dirs
}
#[tauri::command]
pub async fn create_entry(
    name: String,
    path: String,
    flag: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let workdir = super::get_env("workdir".to_string(), app.clone()).await?;
    let path = workdir + path.as_str() + name.as_str();
    println!("{}", flag);
    println!("{}", path);
    let path = std::path::Path::new(&path);
    match flag.as_str() {
        "folder" => std::fs::create_dir(path).map_err(|e| e.to_string())?,
        "file" => std::fs::write(path, "").map_err(|e| e.to_string())?,
        _ => return Err("нет такого флага".to_string()),
    }
    Ok(())
}

#[tauri::command]
pub async fn remove(
    name: String,
    path: String,
    flag: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let workdir = super::get_env("workdir".to_string(), app.clone()).await?;
    let path = workdir + path.as_str() + name.as_str();
    let path = std::path::Path::new(&path);
    match flag.as_str() {
        "folder" => std::fs::remove_dir_all(path).map_err(|e| e.to_string())?,
        "file" => std::fs::remove_file(path).map_err(|e| e.to_string())?,
        _ => return Err("нет такого флага".to_string()),
    }
    Ok(())
}

#[tauri::command]
pub async fn rename(path: String, new_name: String, app: tauri::AppHandle) -> Result<(), String> {
    let workdir = get_env("workdir".to_string(), app.clone()).await.unwrap();
    let path = workdir + path.as_str();
    let new_path = path.replace(path.split("/").last().unwrap(), &new_name);
    let _ = std::fs::rename(&path, new_path);
    Ok(())
}

#[tauri::command]
pub async fn modify_entry(
    before_path: String,
    after_path: String,
    flag: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let workdir = get_env("workdir".to_string(), app.clone()).await.unwrap();
    let before_path = workdir.clone() + before_path.as_str();
    let after_path = workdir + after_path.as_str() + "/" + before_path.split("/").last().unwrap();
    match flag.as_str() {
        "copy" => {
            std::fs::copy(before_path, after_path).map_err(|e| {
                app.emit("error", e.to_string()).unwrap();
                e.to_string()
            })?;
        }
        "move" => {
            std::fs::rename(before_path, after_path).map_err(|e| {
                app.emit("error", e.to_string()).unwrap();
                e.to_string()
            })?;
        }
        _ => (),
    }
    Ok(())
}
