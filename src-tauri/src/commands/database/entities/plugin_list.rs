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
use serde::{Deserialize, Serialize};

use super::{EntityControl, EntityError};

pub enum PluginListFields {
    Name(String),
    Author(String),
    Version(String),
    PluginType(String),
    PluginLink(String),
    Installed(String),
    Enabled(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginList {
    plugin_name: String,
    plugin_author: String,
    plugin_version: String,
    plugin_type: String,
    plugin_link: String,
    is_installed: String,
    is_enabled: String,
}

impl EntityControl<PluginListFields, PluginList> for PluginList {
    fn new(input: Vec<PluginListFields>, app: tauri::AppHandle) -> Result<PluginList, EntityError> {
        if input.len() < 7 {
            return Err(EntityError::WrongInputLength);
        }
        let plugin_name = match input.first() {
            Some(PluginListFields::Name(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Name");
                "?".to_string()
            }
        };
        let plugin_author = match input.get(1) {
            Some(PluginListFields::Author(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Author");
                "?".to_string()
            }
        };
        let plugin_version = match input.get(2) {
            Some(PluginListFields::Version(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Version");
                "?".to_string()
            }
        };
        let plugin_type = match input.get(3) {
            Some(PluginListFields::PluginType(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Type");
                "?".to_string()
            }
        };
        let plugin_link = match input.get(4) {
            Some(PluginListFields::PluginLink(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Link");
                "?".to_string()
            }
        };
        let is_installed = match input.get(5) {
            Some(PluginListFields::Installed(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Installed");
                "?".to_string()
            }
        };
        let is_enabled = match input.last() {
            Some(PluginListFields::Enabled(s)) => s.clone(),
            _ => {
                PluginList::throw_error(app.clone(), "Enabled");
                "false".to_string()
            }
        };
        Ok(PluginList {
            plugin_name,
            plugin_author,
            plugin_version,
            plugin_type,
            plugin_link,
            is_installed,
            is_enabled,
        })
    }

    fn get_value_by_key(&self, key: String) -> Result<String, EntityError> {
        match key.as_str() {
            "name" => Ok(self.plugin_name.clone()),
            "author" => Ok(self.plugin_author.clone()),
            "version" => Ok(self.plugin_version.clone()),
            "type" => Ok(self.plugin_type.clone()),
            "link" => Ok(self.plugin_link.clone()),
            "installed" => Ok(self.is_installed.clone()),
            "enabled" => Ok(self.is_enabled.clone()),
            _ => Err(EntityError::NotFound),
        }
    }
}
