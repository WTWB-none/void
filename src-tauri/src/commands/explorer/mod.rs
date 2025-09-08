use serde::Serialize;
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
#[derive(Serialize)]
pub struct Entry {
    pub name: String,
    pub entry_type: String,
}

impl Entry {
    fn new(name: String, is_dir: bool) -> Self {
        match is_dir {
            true => Entry {
                name,
                entry_type: String::from("dir"),
            },
            false => Entry {
                name,
                entry_type: String::from("file"),
            },
        }
    }
}

#[tauri::command]
pub async fn get_directory_content(dirname: String) -> Result<Vec<Entry>, String> {
    let mut workdir = super::get_env("workdir".to_string()).await.unwrap();
    workdir.push('/');
    workdir.push_str(&dirname);
    workdir.push('/');
    let paths = std::fs::read_dir(workdir.clone()).unwrap();
    let dirs: Vec<Entry> = paths
        .map(|e| {
            let a = e.unwrap();
            (
                a.path().to_str().unwrap().to_string(),
                a.metadata().unwrap().is_dir(),
            )
        })
        .map(|d| {
            let (name, is_dir) = d;
            let name = name.replace(&workdir, "");
            Entry::new(name, is_dir)
        })
        .collect();
    Ok(dirs)
}
