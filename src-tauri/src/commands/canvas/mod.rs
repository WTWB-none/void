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

#[tauri::command]
pub async fn write_canvas_data(
    path: String,
    data: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let workdir = super::get_env("workdir".to_string()).await.unwrap();
    let mut path = path;
    if path.is_empty() {
        let canvas_collection = super::get_directory_content("".to_string()).await;
        let index = canvas_collection
            .unwrap()
            .iter()
            .filter(|dir| dir.name.contains(".canvas"))
            .collect::<Vec<_>>()
            .len()
            + 1;
        path = format!("/untitled{}.canvas", index);
    }
    let path = workdir + path.as_str();
    let path = std::path::Path::new(&path);
    println!("{:#?}", path);
    std::fs::write(path, data.as_str()).map_err(|e| {
        app.emit("error", e.to_string()).unwrap();
        e.to_string()
    })?;
    Ok(path
        .to_str()
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .to_string())
}
