
pub trait StaticBody {
    fn get_position(&self) -> (i32, i32);
}

pub trait PhysicBody {
    fn get_position(&self) -> (i32, i32);
    fn set_velocity(&mut self, val: (i32, i32));
    fn add_velocity(&mut self, val: (i32, i32));
    fn update(&mut self);
}

