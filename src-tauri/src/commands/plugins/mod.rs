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
use serde::Deserialize;
use tauri::Emitter;

use crate::MAIN_FOLDER_PREFIX;

use super::{DB, EntityControl, PluginList, PluginListFields, add_repo};

#[derive(Deserialize)]
struct PluginManifest {
    #[allow(dead_code)]
    repo_type: String,
    member: PluginManifestMember,
}
#[derive(Deserialize)]
struct PluginManifestMember {
    name: String,
    author: String,
    version: String,
    plugin_type: String,
}

#[tauri::command]
pub async fn create_plugins_table(url: String, app: tauri::AppHandle) -> Result<(), String> {
    let db = DB.get().unwrap();
    let linkparts = url.split("/").collect::<Vec<_>>();
    add_repo("Plugins".to_string(), url.clone(), app.clone()).await;
    let client = reqwest::Client::new();
    let manifest = client
        .get(format!(
            "https://raw.githubusercontent.com/{}/{}/main/manifest.json",
            linkparts.get(1).unwrap(),
            linkparts.get(2).unwrap()
        ))
        .send()
        .await;
    let manifest = match manifest {
        Ok(s) => s.text().await.unwrap(),
        Err(e) => {
            app.emit("error", e.to_string()).unwrap();
            "none".to_string()
        }
    };
    let object = serde_json::from_str::<PluginManifest>(&manifest).unwrap();
    let item = vec![
        PluginListFields::Name(object.member.name.clone()),
        PluginListFields::Author(object.member.author),
        PluginListFields::Version(object.member.version),
        PluginListFields::PluginType(object.member.plugin_type),
        PluginListFields::PluginLink(url.clone()),
        PluginListFields::Installed("false".to_string()),
        PluginListFields::Enabled("false".to_string()),
    ];
    db.create::<PluginListFields, PluginList>(
        item,
        app.clone(),
        "plugins_repo",
        object.member.name.as_str(),
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_list_of_plugins(key: String) -> Result<Vec<PluginList>, String> {
    let db = DB.get().unwrap();
    let result = match key.as_str() {
        "installed" => db
            .get_all_members::<PluginList>("plugins_repo")
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|e| {
                e.get_value_by_key("installed".to_string())
                    .unwrap()
                    .as_str()
                    != "false"
            })
            .collect::<Vec<_>>(),
        "not_installed" => db
            .get_all_members::<PluginList>("plugins_repo")
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|p| {
                p.get_value_by_key("installed".to_string())
                    .unwrap()
                    .as_str()
                    == "false"
            })
            .collect::<Vec<_>>(),
        _ => Vec::<PluginList>::new(),
    };
    Ok(result)
}
#[tauri::command]
pub async fn clone_plugin(key: String) -> Result<(), String> {
    let db = DB.get().unwrap();
    let plugin = db
        .get::<PluginList>(key.as_str(), "plugins_repo")
        .await
        .unwrap();
    let plugin_dir = MAIN_FOLDER_PREFIX.get().unwrap().join("plugins");

    let _ = git2::Repository::clone(
        format!(
            "https://{}.git",
            plugin.get_value_by_key("link".to_string()).unwrap()
        )
        .as_str(),
        plugin_dir,
    )
    .map_err(|e| e.to_string())?;
    let extensions = db
        .get_all_members::<PluginList>("plugins_repo")
        .await
        .map_err(|e| e.to_string())?;
    for ext in extensions {
        if ext.get_value_by_key("link".to_string()).unwrap()
            == plugin.get_value_by_key("link".to_string()).unwrap()
        {
            db.update(
                ext.get_value_by_key("plugin_name".to_string()).unwrap(),
                "plugins_repo",
                "is_installed".to_string(),
                "true".to_string(),
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn operate_plugin(plug_name: String, val: String) -> Result<(), String> {
    let db = DB.get().unwrap();
    db.update(plug_name, "plugins_repo", "is_enabled".to_string(), val)
        .await
        .map_err(|e| e.to_string())
}
