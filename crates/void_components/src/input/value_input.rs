/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use std::{cell::RefCell, rc::Rc};

use gpui::{
    Div, EntityId, InteractiveElement, Length, ParentElement, Rgba, SharedString, Styled, blue,
    div, prelude::FluentBuilder, px, rems, rgb,
};

use crate::{
    handlers::{KeyboardHandler, StringManipulationEvent},
    input::{Cursor, Selection},
    traits::Component,
};

#[derive(Default)]
pub struct ValueInput {
    value: Option<Rc<RefCell<Vec<String>>>>,
    placeholder: Option<SharedString>,
    handler: Option<KeyboardHandler>,
    parent_entity_id: Option<EntityId>,
    placeholder_color: Option<Rgba>,
    text_color: Option<Rgba>,
    is_focused: Rc<RefCell<bool>>,
    cursor_position: Option<Rc<RefCell<usize>>>,
    cursor_size: Option<Length>,
    input_size: Option<Length>,
    selection_state: Option<Rc<RefCell<Selection>>>,
    border_color: Option<Rgba>,
    border_focused_color: Option<Rgba>,
}

impl ValueInput {
    pub fn set_init_value(&mut self, value: Rc<RefCell<Vec<String>>>) -> &mut Self {
        self.value = Some(value);
        self
    }
    pub fn set_placeholder(&mut self, placeholder_value: String) -> &mut Self {
        self.placeholder = Some(SharedString::from(placeholder_value));
        self
    }

    pub fn set_handler(&mut self) -> &mut Self {
        self.handler = Some(KeyboardHandler::default());
        self
    }

    pub fn set_parent_id(&mut self, id: EntityId) -> &mut Self {
        self.parent_entity_id = Some(id);
        self
    }

    pub fn set_placeholder_color(&mut self, color: Rgba) -> &mut Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn set_text_color(&mut self, color: Rgba) -> &mut Self {
        self.text_color = Some(color);
        self
    }

    pub fn set_default_focused(&mut self, focused: Rc<RefCell<bool>>) -> &mut Self {
        self.is_focused = focused;
        self
    }
    pub fn init_cursor(&mut self, cursor_state: Rc<RefCell<usize>>) -> &mut Self {
        self.cursor_position = Some(cursor_state);
        self
    }
    pub fn set_cursor_size(&mut self, cursor_size: Length) -> &mut Self {
        self.cursor_size = Some(cursor_size);
        self
    }

    pub fn set_input_size(&mut self, input_size: Length) -> &mut Self {
        self.input_size = Some(input_size);
        self
    }

    pub fn set_selection_state(&mut self, selection_state: Rc<RefCell<Selection>>) -> &mut Self {
        self.selection_state = Some(selection_state);
        self
    }

    pub fn set_border_color(&mut self, color: Rgba) -> &mut Self {
        self.border_color = Some(color);
        self
    }

    pub fn set_focused_border_color(&mut self, color: Rgba) -> &mut Self {
        self.border_focused_color = Some(color);
        self
    }
}

impl Component for ValueInput {
    fn get_styled(&self) -> Div {
        let handler = self.handler.clone().unwrap();
        let entity_id = self.parent_entity_id;
        let focus_remove = self.is_focused.clone();
        let value = self.value.clone().unwrap();
        let cursor_position = self.cursor_position.clone().unwrap();
        let parts = if let Some(s) = self.cursor_position.clone() {
            let temp = value.borrow().clone();
            let temp = temp.split_at(*s.clone().borrow());
            (temp.0.to_vec(), temp.1.to_vec())
        } else {
            (Vec::<String>::new(), Vec::<String>::new())
        };
        let selection = self.selection_state.clone();
        let selection_bounds_modifier = self.selection_state.clone();
        let mouse_bounds_down_modifier = self.selection_state.clone();
        let mouse_bounds_move_modifier = self.selection_state.clone();
        let mouse_cursor_modifier = self.cursor_position.clone();
        let value_len = self.value.as_ref().unwrap().borrow().len();
        div()
            .when_else(
                self.placeholder.is_some()
                    && self.value.is_some()
                    && self.value.clone().unwrap().borrow().is_empty(),
                |el| {
                    el.child(self.placeholder.clone().unwrap())
                        .text_color(self.placeholder_color.unwrap_or(rgb(0x282828)))
                },
                |el| {
                    el.child(
                        div()
                            .children(parts.0.iter().enumerate().map(|(index, part)| {
                                div().child(SharedString::from(part.clone())).when_some(
                                    selection.clone(),
                                    |el, state| {
                                        el.when(
                                            state.borrow().selected_all()
                                                || state.borrow().contains(&index),
                                            |el| el.bg(blue()),
                                        )
                                    },
                                )
                            }))
                            .when(*self.is_focused.borrow(), |el| {
                                el.child(
                                    Cursor::default()
                                        .set_cursor_height(
                                            self.cursor_size.unwrap_or(px(12.).into()),
                                        )
                                        .set_input_size(self.input_size.unwrap_or(rems(3.0).into()))
                                        .get_styled()
                                        .when_some(selection.clone(), |el, state| {
                                            el.when(
                                                state.borrow().selected_all()
                                                    || state
                                                        .borrow()
                                                        .contains(&cursor_position.borrow()),
                                                |el| el.bg(blue()),
                                            )
                                        }),
                                )
                            })
                            .children(parts.1.iter().enumerate().map(|(index, part)| {
                                div().child(SharedString::from(part.clone())).when_some(
                                    selection.clone(),
                                    |el, state| {
                                        el.when(
                                            state.borrow().selected_all()
                                                || state
                                                    .borrow()
                                                    .contains(&(index + *cursor_position.borrow())),
                                            |el| el.bg(blue()),
                                        )
                                    },
                                )
                            }))
                            .on_children_prepainted(move |bounds, _, _| {
                                selection_bounds_modifier
                                    .clone()
                                    .unwrap()
                                    .borrow_mut()
                                    .set_bounds(bounds)
                            })
                            .w_auto()
                            .flex()
                            .flex_wrap()
                            .items_center()
                            .gap(px(0.))
                            .text_color(self.text_color.unwrap_or(rgb(0xffffff))),
                    )
                },
            )
            .when_else(
                *self.is_focused.borrow(),
                |el| el.border_color(self.border_focused_color.unwrap_or(rgb(0x282828))),
                |el| el.border_color(self.border_color.unwrap_or(rgb(0x282828))),
            )
            .on_key_down(move |a, _, c| {
                let ev = handler.on_keydown(a);
                match ev {
                    StringManipulationEvent::AddCharacter(c) => {
                        value
                            .clone()
                            .borrow_mut()
                            .insert(*cursor_position.borrow(), c);
                        *cursor_position.clone().borrow_mut() += 1;
                    }
                    StringManipulationEvent::DeleteCharacter => {
                        if let Some(s) = selection.clone() {
                            if s.borrow().selected_all() {
                                value.clone().borrow_mut().clear();
                                *cursor_position.clone().borrow_mut() = 0;
                                s.borrow_mut().clear();
                            } else if s.borrow().check_range_not_none() {
                                value.borrow_mut().drain(s.borrow().selected_range());
                                let temp_cursor_pos = s.borrow().selected_range();
                                *cursor_position.clone().borrow_mut() = temp_cursor_pos.start;
                                s.borrow_mut().clear();
                            } else if *cursor_position.clone().borrow() > 0 {
                                value
                                    .clone()
                                    .borrow_mut()
                                    .remove(*cursor_position.borrow() - 1);
                                *cursor_position.clone().borrow_mut() -= 1;
                            }
                        }
                    }
                    StringManipulationEvent::RemoveFocus => {
                        if let Some(s) = selection.clone() {
                            if s.borrow().is_active() {
                                s.borrow_mut().clear();
                            } else {
                                *focus_remove.clone().borrow_mut() = false;
                            }
                        }
                    }
                    StringManipulationEvent::MoveCursorLeft => {
                        if *cursor_position.clone().borrow() != 0 {
                            *cursor_position.clone().borrow_mut() -= 1;
                        }
                    }
                    StringManipulationEvent::MoveCursorRight => {
                        if *cursor_position.clone().borrow() < value.clone().clone().borrow().len()
                        {
                            *cursor_position.clone().borrow_mut() += 1;
                            println!("{}", *cursor_position.clone().borrow());
                        }
                    }
                    StringManipulationEvent::SelectAll => {
                        if let Some(s) = selection.clone() {
                            s.borrow_mut().select_all();
                        }
                    }
                    StringManipulationEvent::ShiftSelectLeft => {
                        if let Some(s) = selection.clone() {
                            s.borrow_mut().init_range(*cursor_position.borrow());
                            s.borrow_mut().shift_range_left();
                            s.borrow_mut().check_range_not_zero_length();
                        }
                    }
                    StringManipulationEvent::ShiftSelectRight => {
                        if let Some(s) = selection.clone() {
                            s.borrow_mut().init_range(*cursor_position.borrow());
                            s.borrow_mut().shift_range_right(value.borrow().len());
                            s.borrow_mut().check_range_not_zero_length();
                        }
                    }
                }
                c.notify(entity_id.unwrap());
            })
            .on_any_mouse_down(move |event, _, cx| {
                if let Some(s) = mouse_bounds_down_modifier.clone() {
                    s.borrow_mut().set_drag_start_bounds(event.position);
                    let temp = s.borrow().get_point_index(event.position.x);
                    *mouse_cursor_modifier.as_ref().unwrap().borrow_mut() = temp.min(value_len);
                    cx.notify(entity_id.unwrap());
                }
            })
            .on_mouse_move(move |event, _, cx| {
                if event.dragging() {
                    mouse_bounds_move_modifier
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .modify_drag_range(event.position);
                    cx.notify(entity_id.unwrap());
                }
            })
    }
}
