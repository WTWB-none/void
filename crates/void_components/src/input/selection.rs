use std::{
    cmp::{max, min},
    ops::Range,
};

use gpui::{Bounds, Pixels, Point, px};

#[derive(Clone, Debug)]
pub enum SelectionShiftSide {
    Left,
    Right,
}

#[derive(Default, Clone, Debug)]
pub struct Selection {
    all: bool,
    range: Option<Range<usize>>,
    shift_side: Option<SelectionShiftSide>,
    letters_bounds: Option<Vec<Bounds<Pixels>>>,
    mouse_drag_start: Option<Point<Pixels>>,
}

impl Selection {
    pub fn select_all(&mut self) {
        self.all = true;
        self.range = None;
        self.shift_side = None;
    }
    pub fn set_selection_range(&mut self, selection_range: Range<usize>) {
        self.range = Some(selection_range);
    }
    pub fn clear(&mut self) {
        self.all = false;
        self.range = None;
        self.shift_side = None;
    }
    pub fn init_range(&mut self, cursor_position: usize) {
        match self.range {
            Some(_) => (),
            None => self.range = Some(cursor_position..cursor_position),
        }
    }
    pub fn shift_range_left(&mut self) {
        match self.shift_side {
            Some(_) => (),
            None => self.shift_side = Some(SelectionShiftSide::Left),
        }
        let mut temp = self.selected_range();
        match self.shift_side.clone().unwrap() {
            SelectionShiftSide::Left => {
                if temp.start > 0 {
                    temp.start -= 1;
                }
            }
            SelectionShiftSide::Right => {
                if temp.end > 0 {
                    temp.end -= 1;
                }
            }
        };
        self.range = Some(temp.start..temp.end)
    }

    pub fn shift_range_right(&mut self, text_bounds: usize) {
        match self.shift_side {
            Some(_) => (),
            None => self.shift_side = Some(SelectionShiftSide::Right),
        }
        let mut temp = self.selected_range();
        match self.shift_side.clone().unwrap() {
            SelectionShiftSide::Left => {
                if temp.start < text_bounds {
                    temp.start += 1;
                }
            }
            SelectionShiftSide::Right => {
                if temp.end < text_bounds {
                    temp.end += 1;
                }
            }
        };
        self.range = Some(temp.start..temp.end);
    }

    pub fn check_range_not_zero_length(&mut self) {
        let temp = self.selected_range();
        if temp.start == temp.end {
            self.clear();
        }
    }

    pub fn check_range_not_none(&self) -> bool {
        self.range.is_some()
    }

    pub fn selected_all(&self) -> bool {
        self.all
    }

    pub fn is_active(&self) -> bool {
        self.selected_all() || self.check_range_not_none()
    }

    pub fn contains(&self, index: &usize) -> bool {
        self.range.clone().unwrap_or(0..0).contains(index)
    }

    pub fn selected_range(&self) -> Range<usize> {
        self.range.clone().unwrap()
    }

    pub fn set_bounds(&mut self, bounds: Vec<Bounds<Pixels>>) {
        self.letters_bounds = Some(bounds);
    }

    pub fn set_drag_start_bounds(&mut self, bounds: Point<Pixels>) {
        self.mouse_drag_start = Some(bounds);
    }
    pub fn modify_drag_range(&mut self, bounds: Point<Pixels>) {
        let Some(start) = self.mouse_drag_start.as_ref() else {
            return;
        };
        let first_blank = self.get_point_index(min(bounds.x, start.x));
        let second_blank = self.get_point_index(max(bounds.x, start.x));
        self.range = Some(first_blank..second_blank);
    }

    pub fn get_point_index(&self, point: Pixels) -> usize {
        let Some(bounds) = self.letters_bounds.as_ref() else {
            return 0;
        };
        let mut out: usize = 0;
        for (index, bound) in bounds.iter().enumerate() {
            let mid = bound.left() + ((bound.size.width) / 2.);
            if mid <= point || bound.left() <= point {
                out = index;
            }
        }
        out
    }
}
