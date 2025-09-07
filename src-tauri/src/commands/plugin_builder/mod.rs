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
use super::get_env;

#[tauri::command]
pub async fn read_plugin(name: String) -> String {
    let workspace = get_env("workdir".to_string()).await.unwrap();
    let path = format!("{}/.conf/plugins/{}/dist/plugin.js", workspace, name);
    let path = std::path::Path::new(&path);
    std::fs::read_to_string(path).unwrap()
}
