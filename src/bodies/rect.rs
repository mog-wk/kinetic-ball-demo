#![allow(unused)]

use crate::config;
use crate::utils::shapes;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;

use crate::bodies::physics;

pub struct Rect {
    // physic Options
    position: (i32, i32), // top-left corner
    width: u32,
    height: u32,

    velocity: (i32, i32),

    // render options
    fill_color: Color,
    border_color: Color,

}

impl Rect {
    fn update(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
    }

}

impl std::default::Default for Rect {
    fn default() -> Self {
        Self {
            position: (0, 0),
            width: 32,
            height: 32,

            velocity: (0, 0),

            fill_color: Color::RGB(0, 127, 0),
            border_color: Color::RGB(127, 127, 127),
        }
    }
}

pub struct CanvasBox {
    rect: Rect,
    visible: bool,
}

impl CanvasBox {
    fn invisible() -> Self {
        Self {
            rect: Rect::default(),
            visible: false,
        }
    }
}

impl std::default::Default for CanvasBox {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            visible: true,
        }
    }
}

impl physics::StaticBody for CanvasBox {
    fn get_position(&self) -> (i32, i32) {
        self.rect.position
    }
    fn update(&self) {
        // legacy should not be called in static body
    }
}
