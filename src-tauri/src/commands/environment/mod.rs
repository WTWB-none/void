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
use crate::{
    MAIN_FOLDER_PREFIX,
    commands::database::{DB, EntityError, MainConfig},
};

use super::EntityControl;

#[tauri::command]
pub async fn get_env(ename: String, _app: tauri::AppHandle) -> Result<String, String> {
    match DB
        .get()
        .unwrap()
        .get::<Option<MainConfig>>("singletone", "main_config")
        .await
    {
        Ok(Some(data)) => match data.get_value_by_key(ename) {
            Ok(str) => Ok(str),
            Err(e) => Err(e.to_string()),
        },
        Ok(None) => Err(EntityError::NotFound.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn set_env(ename: String, name: String) -> Result<(), String> {
    match DB
        .get()
        .unwrap()
        .update("singletone".to_string(), "main_config", ename, name)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_app_config_dir() -> Result<String, String> {
    Ok(MAIN_FOLDER_PREFIX
        .get()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string())
}
