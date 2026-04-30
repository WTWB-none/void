/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gpui::KeyDownEvent;

#[derive(Default, Clone)]
pub struct KeyboardHandler {}

pub enum StringManipulationEvent {
    DeleteCharacter,
    SelectAll,
    RemoveFocus,
    MoveCursorLeft,
    MoveCursorRight,
    AddCharacter(String),
    ShiftSelectLeft,
    ShiftSelectRight,
}

impl KeyboardHandler {
    pub fn on_keydown(&self, event: &KeyDownEvent) -> StringManipulationEvent {
        match event.keystroke.key.as_str() {
            "backspace" => StringManipulationEvent::DeleteCharacter,
            "space" => StringManipulationEvent::AddCharacter(" ".to_string()),
            "escape" => StringManipulationEvent::RemoveFocus,
            "left" => {
                if event.keystroke.modifiers.shift {
                    StringManipulationEvent::ShiftSelectLeft
                } else {
                    StringManipulationEvent::MoveCursorLeft
                }
            }
            "right" => {
                if event.keystroke.modifiers.shift {
                    StringManipulationEvent::ShiftSelectRight
                } else {
                    StringManipulationEvent::MoveCursorRight
                }
            }
            "a" => {
                if event.keystroke.modifiers.control {
                    StringManipulationEvent::SelectAll
                } else {
                    StringManipulationEvent::AddCharacter(
                        event
                            .keystroke
                            .key_char
                            .as_ref()
                            .unwrap_or(&"".to_string())
                            .to_string(),
                    )
                }
            }
            _ => StringManipulationEvent::AddCharacter(
                event
                    .keystroke
                    .key_char
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            ),
        }
    }
}
