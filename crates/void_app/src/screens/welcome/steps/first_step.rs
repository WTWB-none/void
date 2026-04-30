/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{cell::RefCell, path::Path, rc::Rc};

use gpui::{
    EntityId, FocusHandle, FontWeight, InteractiveElement, ParentElement, SharedString, Styled,
    div, img, prelude::FluentBuilder, px, rems, rgb, rgba, white,
};
use void_components::{
    input::{Selection, ValueInput},
    traits::Component,
};

use crate::screens::WelcomeEntities;

pub struct FirstStep {
    username: Rc<RefCell<Vec<String>>>,
    focus_handler: Option<FocusHandle>,
    id: Option<EntityId>,
    input_focus_state: Rc<RefCell<bool>>,
    input_cursor_state: Rc<RefCell<usize>>,
    input_selection_state: Option<Rc<RefCell<Selection>>>,
}
//WIP: comments on their way
impl FirstStep {
    pub fn new() -> Self {
        FirstStep {
            username: Rc::new(RefCell::new(Vec::<String>::new())),
            focus_handler: None,
            id: None,
            input_focus_state: Rc::new(RefCell::new(false)),
            input_cursor_state: Rc::new(RefCell::new(0)),
            input_selection_state: None,
        }
    }
    pub fn get_element(
        &mut self,
        _cx: &mut gpui::Context<WelcomeEntities>,
    ) -> impl gpui::IntoElement {
        match self.focus_handler {
            Some(_) => {
                if !*self.input_focus_state.borrow() {
                    self.focus_handler = None
                }
            }
            None => self.focus_handler = Some(_cx.focus_handle()),
        };
        match self.id {
            Some(_) => (),
            None => self.id = Some(_cx.entity_id()),
        };
        match self.input_selection_state {
            Some(_) => (),
            None => self.input_selection_state = Some(Rc::new(RefCell::new(Selection::default()))),
        }
        let focus_state = self.input_focus_state.clone();
        let self_id = self.id;
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(rems(1.1))
            .text_color(white())
            .child(
                img(Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("public")
                    .join("128x128@2x.png"))
                .size(rems(10.))
                .border(px(2.))
                .border_color(rgba(0xB8698FFF))
                .rounded(px(30.)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .gap(rems(0.3))
                    .child(
                        div()
                            .text_color(rgba(0xB8698FFF))
                            .font_weight(FontWeight::BOLD)
                            .text_size(rems(2.3))
                            .child(SharedString::new("Welcome to VOID!")),
                    )
                    .child(
                        div()
                            .text_color(rgba(0xFFFFFF66))
                            .child(SharedString::new("Your second brain note-taking app")),
                    ),
            )
            .child(
                ValueInput::default()
                    .set_placeholder("name".to_string())
                    .set_init_value(self.username.clone())
                    .set_handler()
                    .set_parent_id(self.id.unwrap())
                    .set_default_focused(self.input_focus_state.clone())
                    .init_cursor(self.input_cursor_state.clone())
                    .set_cursor_size(rems(1.3).into())
                    .set_selection_state(self.input_selection_state.clone().unwrap())
                    .set_border_color(rgb(0x282828))
                    .set_focused_border_color(rgb(0xffffff))
                    .set_input_size(rems(2.1).into())
                    .get_styled()
                    .text_size(rems(1.3))
                    .w(rems(20.))
                    .h(rems(3.))
                    .bg(rgb(0x000000))
                    .border(px(1.))
                    .rounded(rems(0.5))
                    .p(px(5.))
                    .on_any_mouse_down(move |_, _, cx| {
                        *focus_state.clone().borrow_mut() = true;
                        cx.notify(self_id.unwrap());
                    })
                    .when_some(self.focus_handler.clone(), |el, handler| {
                        el.track_focus(&handler)
                    }),
            )
    }
}
