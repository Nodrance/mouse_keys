use crate::settings::{MouseMode, AccelerationType};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MouseMotion {
    acceleration_type: AccelerationType,
    direction: Direction,
    starting_speed: f32,
    acceleration: f32,

    current_speed: f32,
    frames_active: usize,
}

impl MouseMotion {
    pub fn new(
        modestate: MouseMode, 
        direction: Direction,
    ) -> Self {
        Self { 
            acceleration_type: modestate.acceleration_type, 
            direction, 
            starting_speed: modestate.default_speed, 
            acceleration: modestate.acceleration, 
            current_speed: modestate.default_speed, 
            frames_active: 0,
        }
    }
    pub fn increment_speed(&mut self) {
        self.frames_active += 1;
        match self.acceleration_type {
            AccelerationType::Linear => {}
            AccelerationType::Quadratic => {
                self.current_speed += self.acceleration
            }
            AccelerationType::Exponential => {
                self.current_speed *= self.acceleration
            }
        }
        if self.frames_active == 1 {
            println!("Zero")
        }
    }
    pub fn get_delta(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => {(0, -self.current_speed as isize)}
            Direction::Down => {(0, self.current_speed as isize)}
            Direction::Left => {(-self.current_speed as isize, 0)}
            Direction::Right => {(self.current_speed as isize, 0)}
        }
    }
}
