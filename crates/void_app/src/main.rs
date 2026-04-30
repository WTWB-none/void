/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use gpui::{AppContext, Application, WindowOptions};

use crate::screens::WelcomeOperator;

mod screens;
fn main() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| WelcomeOperator::default())
        })
        .unwrap();
    });
}
