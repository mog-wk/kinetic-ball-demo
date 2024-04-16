use crate::config;
use crate::utils::shapes;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;

//#[derive(PhysicBody)]
#[derive(Debug)]
pub struct Sphere {
    // format options
    position: (i32, i32), // center position
    radius: i32,

    // physic options
    velocity: (i32, i32),
    acceleration: (i32, i32),

    // render options
    fill_color: Color,
    border_color: Color,
}
impl Sphere {
    pub fn update(&mut self) {
        self.add_position(self.velocity);
        self.add_velocity(self.acceleration);
        self.set_acceleration((0, 0));
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.fill_color);
        shapes::draw_circle(canvas, self.position.0, self.position.1, self.radius);
        canvas.set_draw_color(self.border_color);
        shapes::draw_circunference(canvas, self.position.0, self.position.1, self.radius);
    }

    pub fn set_position(&mut self, val: (i32, i32)) {
        self.position = val;
    }
    pub fn add_position(&mut self, val: (i32, i32)) {
        self.position = (self.position.0 + val.0, self.position.1 + val.1);
    }
    pub fn set_velocity(&mut self, val: (i32, i32)) {
        self.velocity = val;
    }
    pub fn add_velocity(&mut self, val: (i32, i32)) {
        self.velocity = (self.velocity.0 + val.0, self.velocity.1 + val.1);
    }
    pub fn set_acceleration(&mut self, val: (i32, i32)) {
        self.acceleration = val;
    }
    pub fn add_acceleration(&mut self, val: (i32, i32)) {
        self.acceleration = (self.acceleration.0 + val.0, self.acceleration.1 + val.1);
    }
}

impl std::default::Default for Sphere {
    fn default() -> Self {
        Self {
            position: (
                config::Window::WIDTH as i32 / 2,
                config::Window::HEIGHT as i32 / 2,
            ),
            radius: 48,
            velocity: (0, 0),
            acceleration: (0, 0),
            fill_color: Color::RGB(127, 127, 127),
            border_color: Color::RGB(127, 127, 127),
        }
    }
}

pub fn new() -> Sphere {
    Sphere::default()
}
