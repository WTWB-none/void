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
mod entities;

pub use entities::*;
use once_cell::sync::OnceCell;
use surrealdb::{Surreal, engine::local::RocksDb};

use crate::MAIN_FOLDER_PREFIX;
pub static DB: OnceCell<DbRepo> = OnceCell::new();

pub async fn init() {
    let db_path = format!("{}/db", MAIN_FOLDER_PREFIX.get().unwrap().to_str().unwrap());
    let db = Surreal::new::<RocksDb>(db_path).await.unwrap();
    db.use_ns("mindbreaker").use_db("config").await.unwrap();
    let db = DbRepo::new(db);
    DB.set(db).map_err(|_| EntityError::DbQueryError).unwrap();
}

pub async fn init_official_plugins(app: tauri::AppHandle) -> Result<(), String> {
    let db = DB.get().unwrap();
    let official_plugins = vec![
        "headers",
        "callout",
        "code-block",
        "inline",
        "lists",
        "page-breaker",
        "quote",
        "tags",
        "line-numbers",
    ];
    for plugin in official_plugins {
        let plug = vec![
            PluginListFields::Name(plugin.to_string()),
            PluginListFields::Author("Transhumanist".to_string()),
            PluginListFields::Version("latest".to_string()),
            PluginListFields::PluginType("official".to_string()),
            PluginListFields::PluginLink("none".to_string()),
            PluginListFields::Installed("true".to_string()),
            PluginListFields::Enabled("true".to_string()),
        ];
        db.create::<PluginListFields, PluginList>(plug, app.clone(), "plugins_repo", plugin)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn create_first_database(app: tauri::AppHandle) -> Result<(), String> {
    init().await;
    match DB
        .get()
        .unwrap()
        .get::<MainConfig>("singletone", "main_config")
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            let input = vec![
                MainConfigFields::Name("".to_string()),
                MainConfigFields::FirstRun("true".to_string()),
                MainConfigFields::Workdir("".to_string()),
            ];
            DB.get()
                .unwrap()
                .create::<MainConfigFields, MainConfig>(
                    input,
                    app.clone(),
                    "main_config",
                    "singletone",
                )
                .await
                .map_err(|e| e.to_string())?;
            init_official_plugins(app).await
        }
    }
}
