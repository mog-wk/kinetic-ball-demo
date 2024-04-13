use crate::config;
use crate::utils::shapes;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;

//#[derive(PhysicBody)]
pub struct Sphere {
    // physic options
    center_pos: (i32, i32),
    radius: i32,
    velocity: (i32, i32),

    // render options
    fill_color: Color,
    border_color: Color,
}
impl Sphere {
    pub fn update(&mut self) {
        self.center_pos = (
            self.center_pos.0 + self.velocity.0,
            self.center_pos.1 + self.velocity.1,
        );

        //self.velocity = (self.drag.0, self.drag.1);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.fill_color);
        shapes::draw_circle(canvas,
            self.center_pos.0,
            self.center_pos.1,
            self.radius
        );
        canvas.set_draw_color(self.border_color);
        shapes::draw_circunference(canvas,
            self.center_pos.0,
            self.center_pos.1,
            self.radius
        );
    }

    pub fn set_velocity(&mut self, val: (i32, i32)) {
        self.velocity = val;
    }
    pub fn add_velocity(&mut self, val: (i32, i32)) {
        self.velocity = (self.velocity.0 + val.0, self.velocity.1 + val.1);
    }
}

impl std::default::Default for Sphere {
    fn default() -> Self {
        Self {
            center_pos: (
                config::Window::WIDTH as i32 / 2,
                config::Window::HEIGHT as i32 / 2,
            ),
            radius: 48,
            velocity: (0, 0),
            fill_color: Color::RGB(127, 127, 127),
            border_color: Color::RGB(127, 127, 127),
        }
    }
}

pub fn new() -> Sphere {
    Sphere::default()
}
