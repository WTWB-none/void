/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;

#[derive(Default)]
pub struct GlobalConfig {
    scope: Option<PathBuf>,
}

impl GlobalConfig {
    /// changes global fs scope
    pub fn change_scope(&mut self, link: PathBuf) {
        self.scope = Some(link);
    }

    /// returns global fs scope
    pub fn get_scope(&mut self) -> Option<PathBuf> {
        self.scope.clone()
    }
}
