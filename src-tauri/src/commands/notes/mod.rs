use crate::commands::get_env;

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
#[tauri::command]
pub async fn get_note_content(path: String) -> Result<String, String> {
    let workdir = std::path::PathBuf::from(get_env("workdir".to_string()).await.unwrap());
    let path = workdir.join(&path);
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(content)
}

#[tauri::command]
pub async fn write_note_changes(path: String, value: String) -> Result<(), String> {
    let workdir = std::path::PathBuf::from(get_env("workdir".to_string()).await.unwrap());
    let path = workdir.join(&path);
    std::fs::write(path, value).map_err(|e| e.to_string())?;
    Ok(())
}
