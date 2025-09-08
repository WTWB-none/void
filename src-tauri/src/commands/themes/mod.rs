use crate::MAIN_FOLDER_PREFIX;

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
use super::{DB, EntityControl, EntityError, ThemeRepo, ThemeRepoField, add_repo};
use serde::Deserialize;
use std::{fs, vec};
use tauri::Emitter;

#[derive(Deserialize, Debug)]
struct ThemeManifest {
    #[allow(dead_code)]
    repo_type: String,
    members: Vec<ThemeManifestMember>,
}

#[derive(Deserialize, Debug)]
struct ThemeManifestMember {
    name: String,
    author: String,
    version: String,
}

#[tauri::command]
pub async fn get_theme(name: String, _app: tauri::AppHandle) -> Result<String, String> {
    let theme = MAIN_FOLDER_PREFIX
        .get()
        .unwrap()
        .join("themes")
        .join(name)
        .join("theme.css");
    fs::read_to_string(theme).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_list_of_themes(key: String) -> Result<Vec<ThemeRepo>, String> {
    let db = DB.get().unwrap();
    let entities = db
        .get_all_members::<ThemeRepo>("themes_repo")
        .await
        .map_err(|e| e.to_string())?;
    match key.as_str() {
        "installed" => Ok(entities
            .into_iter()
            .filter(move |t| {
                t.get_value_by_key("installed".to_string())
                    .unwrap()
                    .as_str()
                    == "true"
            })
            .collect::<Vec<ThemeRepo>>()),
        "not_installed" => Ok(entities
            .into_iter()
            .filter(|t| {
                t.get_value_by_key("installed".to_string())
                    .unwrap()
                    .as_str()
                    == "false"
            })
            .collect::<Vec<ThemeRepo>>()),
        _ => Ok(Vec::<ThemeRepo>::new()),
    }
}

#[tauri::command]
pub async fn create_themes_table(link: String, app: tauri::AppHandle) {
    let db = DB.get().unwrap();
    add_repo("Theme".to_string(), link.clone(), app.clone()).await;
    let linkparts = link.split('/').collect::<Vec<&str>>();
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://raw.githubusercontent.com/{}/{}/main/manifest.json",
            linkparts.get(1).unwrap(),
            linkparts.get(2).unwrap()
        ))
        .send()
        .await;
    let manifest = match response {
        Ok(r) => r.text().await.unwrap(),
        _ => "none".to_string(),
    };
    let themes = serde_json::from_str::<ThemeManifest>(&manifest).unwrap();
    for theme in themes.members.into_iter() {
        let link = format!(
            "https://raw.githubusercontent.com/{}/{}/main/{}/theme.css",
            linkparts.get(1).unwrap(),
            linkparts.get(2).unwrap(),
            theme.name
        );
        let input: Vec<ThemeRepoField> = vec![
            ThemeRepoField::Name(theme.name.clone()),
            ThemeRepoField::Author(theme.author.clone()),
            ThemeRepoField::Version(theme.version.clone()),
            ThemeRepoField::Link(link),
            ThemeRepoField::Installed("false".to_string()),
        ];
        db.create::<ThemeRepoField, ThemeRepo>(input, app.clone(), "themes_repo", &theme.name)
            .await
            .unwrap();
    }
}

#[tauri::command]
pub async fn clone_theme(key: String, app: tauri::AppHandle) -> Result<(), String> {
    let db = DB.get().unwrap();
    let themes_list = db
        .get_all_members::<ThemeRepo>("themes_repo")
        .await
        .map_err(|e| e.to_string())?;
    let mut selected_theme: Option<ThemeRepo> = None;
    for theme in themes_list {
        if theme
            .get_value_by_key("name".to_string())
            .map_err(|e| e.to_string())?
            == key
        {
            selected_theme = Some(theme);
        }
    }
    match selected_theme {
        Some(theme) => {
            let db = DB.get().unwrap();
            let client = reqwest::Client::new();
            let theme_css = client
                .get(theme.get_value_by_key("link".to_string()).unwrap())
                .send()
                .await
                .map_err(|e| e.to_string())?;
            db.update(
                key.clone(),
                "themes_repo",
                "is_installed".to_string(),
                "true".to_string(),
            )
            .await
            .map_err(|e| e.to_string())?;
            let theme_css = theme_css.text().await.map_err(|e| e.to_string())?;
            let theme_dir = MAIN_FOLDER_PREFIX
                .get()
                .unwrap()
                .join("themes")
                .join(key.clone());
            let theme_file = theme_dir.join("theme.css");
            fs::create_dir(theme_dir).map_err(|e| e.to_string())?;
            fs::write(theme_file, theme_css).map_err(|e| e.to_string())?;
        }
        None => return Err(EntityError::NotFound.to_string()),
    }
    app.emit("theme_downloaded", key).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn check_theme_update(theme_name: String, app: tauri::AppHandle) -> Result<(), String> {
    let db = DB.get().unwrap();
    let theme = db
        .get::<ThemeRepo>(&theme_name, "themes_repo")
        .await
        .map_err(|e| e.to_string())?;
    let fetch_client = reqwest::Client::new();
    let css = fetch_client
        .get(theme.get_value_by_key("link".to_string()).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let prev_theme = get_theme(theme_name.clone(), app.clone()).await.unwrap();
    if prev_theme != css {
        let theme_path = MAIN_FOLDER_PREFIX
            .get()
            .unwrap()
            .join("themes")
            .join(theme_name)
            .join("theme.css");
        std::fs::write(theme_path, css).unwrap();
    } else {
        app.emit("notify", "Установлена последняя версия темы")
            .unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_theme(theme_name: String, app: tauri::AppHandle) -> Result<(), String> {
    let db = DB.get().unwrap();
    let theme_dir = MAIN_FOLDER_PREFIX
        .get()
        .unwrap()
        .join("themes")
        .join(&theme_name);
    std::fs::remove_dir_all(theme_dir).map_err(|e| e.to_string())?;
    db.update(
        theme_name,
        "themes_repo",
        "is_installed".to_string(),
        "false".to_string(),
    )
    .await
    .map_err(|e| e.to_string())?;
    app.emit("theme_changed", "").unwrap();
    Ok(())
}
