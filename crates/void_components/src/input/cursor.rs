/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gpui::{Length, ParentElement, Styled, div, prelude::FluentBuilder, px, rems, rgb};

use crate::traits::Component;

#[derive(Default)]
pub struct Cursor {
    cursor_height: Option<Length>,
    input_size: Option<Length>,
}

impl Cursor {
    pub fn set_cursor_height(&mut self, height: Length) -> &mut Self {
        self.cursor_height = Some(height);
        self
    }

    pub fn set_input_size(&mut self, input_size: Length) -> &mut Self {
        self.input_size = Some(input_size);
        self
    }
}

impl Component for Cursor {
    fn get_styled(&self) -> gpui::Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(0.5))
                    .when_else(
                        self.cursor_height.is_some(),
                        |el| el.h(self.cursor_height.unwrap()),
                        |el| el.h_full(),
                    )
                    .bg(rgb(0xffffff)),
            )
            .h(self.input_size.unwrap_or(rems(3.).into()))
            .w(px(1.))
    }
}
