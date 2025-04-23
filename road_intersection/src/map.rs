use std::rc::Rc;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::settings::Settings;

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Path {
        Path { start: Point::new(start.0, start.1), end: Point::new(end.0, end.1) }
    }
}

