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
mod commands;
use std::path::PathBuf;

use commands::*;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

pub static MAIN_FOLDER_PREFIX: once_cell::sync::OnceCell<PathBuf> =
    once_cell::sync::OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let video_state = init_video_server();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                std::fs::create_dir_all(
                    app.handle()
                        .path()
                        .home_dir()
                        .unwrap()
                        .join(".config")
                        .join("void"),
                )
                .unwrap();
                crate::MAIN_FOLDER_PREFIX
                    .set(
                        app.handle()
                            .path()
                            .home_dir()
                            .unwrap()
                            .join(".config")
                            .join("void"),
                    )
                    .unwrap();
                app.fs_scope()
                    .allow_directory(MAIN_FOLDER_PREFIX.get().unwrap(), true)
                    .unwrap();
                if let Err(e) = create_first_database(app.handle().clone()).await {
                    eprintln!("Ошибка при инициализации{}", e);
                }
                if get_env("workdir".to_string()).await.unwrap().as_str() != "" {
                    let scope = app.fs_scope();
                    let _ =
                        scope.allow_directory(get_env("workdir".to_string()).await.unwrap(), true);
                }
            });
            Ok(())
        })
        .manage(PtyState::default())
        .manage(video_state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_env,
            set_env,
            get_settings_list,
            get_file,
            open_neovim,
            resize_neovim,
            send_to_neovim,
            create_first_database,
            get_theme,
            get_list_of_themes,
            read_plugin,
            create_themes_table,
            clone_theme,
            add_repo,
            get_repos_list,
            delete_repo,
            check_theme_update,
            delete_theme,
            get_directory_content,
            create_entry,
            remove,
            setup_config_directory,
            allow_scope,
            read_audio_metadata,
            get_video_url,
            set_video_path,
            check_file_exists,
            get_note_content,
            rename,
            write_canvas_data,
            modify_entry,
            create_plugins_table,
            get_list_of_plugins,
            clone_plugin,
            write_note_changes,
            operate_plugin,
            get_config_directory,
            copy_font,
            get_all_user_fonts,
            get_app_config_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
