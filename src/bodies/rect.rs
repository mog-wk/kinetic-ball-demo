#![allow(unused)]

use crate::config;
use crate::utils::shapes;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;

use crate::bodies::physics;

pub struct StaticRect {
    // physic Options
    position: (i32, i32), // top-left corner
    width: u32,
    height: u32,

    // render options
    fill_color: Color,
    border_color: Color,

}

impl StaticRect {
    pub fn render_border(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.border_color);
        canvas.draw_rect(
            sdl2::rect::Rect::new(self.position.0, self.position.1, self.width, self.height)
        );

    }
}

impl physics::StaticBody for StaticRect {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}

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
    pub fn update(&mut self) {
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
    rect: StaticRect,
    visible: bool,
}

impl CanvasBox {
    fn invisible() -> Self {
        Self {
            rect: Self::default_static_rect(),
            visible: false,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        self.rect.render_border(canvas);
    }

    fn default_static_rect() -> StaticRect {
        let unit = (config::Window::WIDTH / 10, config::Window::HEIGHT / 10);
        let position = (unit.0 as i32, unit.1 as i32);
        let width = unit.0 * 8;
        let height = unit.1 * 8;
        StaticRect {
            position,
            width,
            height,
            fill_color: Color::RGB(0, 127, 0),
            border_color: Color::RGB(127, 127, 127),
        }
    }
}

impl std::default::Default for CanvasBox {
    fn default() -> Self {
        Self {
            rect: Self::default_static_rect(),
            visible: true,
        }
    }
}

impl physics::StaticBody for CanvasBox {
    fn get_position(&self) -> (i32, i32) {
        self.rect.position
    }
}
