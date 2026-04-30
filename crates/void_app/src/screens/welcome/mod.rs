/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use gpui::{
    AnyEntity, AppContext, Font, FontFallbacks, FontFeatures, FontWeight, ParentElement, Render,
    SharedString, Styled, div, rgba,
};
mod steps;
use steps::FirstStep;

pub enum WelcomeEntities {
    First(FirstStep),
}

impl Render for WelcomeEntities {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        match self {
            WelcomeEntities::First(s) => s.get_element(cx),
        }
    }
}

#[derive(Default)]
pub struct WelcomeOperator {
    step: Option<Box<AnyEntity>>,
}

impl Render for WelcomeOperator {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        match self.step {
            Some(_) => (),
            None => {
                let screen = cx.new(|_| WelcomeEntities::First(FirstStep::new()));
                self.step = Some(Box::new(screen.into_any()));
            }
        }
        div()
            .font(Font {
                family: SharedString::new_static(".SystemUIFont"),
                features: FontFeatures::default(),
                fallbacks: Some(FontFallbacks::default()),
                weight: FontWeight::NORMAL,
                style: gpui::FontStyle::Normal,
            })
            .bg(rgba(0x1A1A24FF))
            .size_full()
            .child(
                self.step
                    .clone()
                    .unwrap()
                    .downcast::<WelcomeEntities>()
                    .unwrap(),
            )
    }
}
