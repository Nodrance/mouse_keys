use std::io::{self, Write};

use macroquad::prelude::*;
use mouse_rs::Mouse;
use mouse_rs::types::keys::Keys;


// Key used to activate
const ACTIVATE_KEY: KeyCode = KeyCode::GraveAccent;
// Toggle or hold
const ACTIVATE_MODE: ActivationMode = ActivationMode::Toggle;
// const ACTIVATE_MODE: ActivationMode = ActivationMode::Hold;

// Mouse button keys, feel free to change. https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
const LEFT_CLICK_KEY: KeyCode = KeyCode::Space;
const RIGHT_CLICK_KEY: KeyCode = KeyCode::Enter;
const MIDDLE_CLICK_KEY: KeyCode = KeyCode::Backslash;
const MOUSE_BUTTON_MODE: ActivationMode = ActivationMode::Hold;

const SCROLL_DOWN_KEY: KeyCode = KeyCode::Comma;
const SCROLL_UP_KEY: KeyCode = KeyCode::Slash;
const SCROLL_STEPS: i32 = 1; // Lines per key press, cannot currently be adjusted while using


// Movement buttons

const MOVE_LEFT_KEY: KeyCode = KeyCode::Left;
const MOVE_UP_KEY: KeyCode = KeyCode::Up;
const MOVE_RIGHT_KEY: KeyCode = KeyCode::Right;
const MOVE_DOWN_KEY: KeyCode = KeyCode::Down;
const SPEED_DOWN_KEY: KeyCode = KeyCode::Minus;
const SPEED_UP_KEY: KeyCode = KeyCode::Equal;

const MOVEMENT_MODE_KEY: KeyCode = KeyCode::Tab;
const RESET_KEY: KeyCode = KeyCode::Backspace; // Resets speed, movement mode, and also held mouse buttons if toggle mode is on
const BIG_STEP_KEY: KeyCode = KeyCode::LeftControl; // While held, movement acts as if speed is 2x, and steps jump by 4x (so instead of 2.5 meaning "add 2.5", it'll add 10)
const SMALL_STEP_KEY: KeyCode = KeyCode::LeftAlt;
const DEFAULT_SPEED: f32 = 5.0;
const SPEED_STEP: f32 = 2.5;


// Hold this and press a slot key to save the mouse position. Only works while mouse mode is active
const SAVE_KEY: KeyCode = KeyCode::LeftShift;
// Press a slot key to load the mouse position. Make sure to change the 10 if you add or remove from the list
const SLOT_COUNT: usize = 10;
const SLOT_KEYS: [KeyCode; SLOT_COUNT] = [
    KeyCode::Key0,
    //KeyCode::Kp0 // Keypad or numpad 0
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

enum ActivationMode {
    Toggle,
    Hold,
}

// Movement options
#[derive(PartialEq, Eq)]
enum MovementMode {
    Linear,
    Quadratic,
    Exponential,
    SuperExponential,
}

impl MovementMode {
    fn next(&mut self) {
        match self {
            MovementMode::Linear => {*self = MovementMode::Quadratic;}
            MovementMode::Quadratic => {*self = MovementMode::Exponential;}
            MovementMode::Exponential => {*self = MovementMode::SuperExponential;}
            MovementMode::SuperExponential => {*self = MovementMode::Linear;}
        }
    }
}

fn toggle_mouse(mouse: &Mouse, key: Keys, state: bool) {
    println!("Setting mouse key {:?} to state {}", key, state);
    if state {
        mouse.release(&key).unwrap();
    }
    else {
        mouse.press(&key).unwrap();
    }
}

fn move_mouse_by(mouse: &Mouse, delta_x: i32, delta_y: i32) {
    let old_position = mouse.get_position().unwrap();
    let new_x = old_position.x + delta_x;
    let new_y = old_position.y + delta_y;
    mouse.move_to(new_x, new_y).unwrap();
}

#[macroquad::main("Mouse Tool")]
async fn main() {
    let mut speed = DEFAULT_SPEED;
    let mut step_size_x = speed;
    let mut step_size_y = speed;
    let mut saved_positions = [(0,0); SLOT_COUNT];
    let mut mouse_enabled = false;
    let mut mouse_state = (false, false, false);
    let mut movement_mode = MovementMode::Linear;
    let mouse = Mouse::new();
    // let mut stdout = io::stdout().lock();
    println!("Started");
    loop {
        // Is this thing on?
        match ACTIVATE_MODE {
            ActivationMode::Hold => {
                mouse_enabled = is_key_down(ACTIVATE_KEY)
            }
            ActivationMode::Toggle => {
                if is_key_pressed(ACTIVATE_KEY) {
                    mouse_enabled = !mouse_enabled
                }
            }
        }
        // write!(&mut stdout, "{}", mouse_enabled).unwrap();
        // stdout.flush();
        if !mouse_enabled {
            if mouse_state.0 {mouse.release(&Keys::LEFT).unwrap();}
            if mouse_state.1 {mouse.release(&Keys::MIDDLE).unwrap();}
            if mouse_state.2 {mouse.release(&Keys::RIGHT).unwrap();}
            mouse_state = (false, false, false);
            next_frame().await;
            continue;
        }

        // Mouse buttons
        match MOUSE_BUTTON_MODE {
            ActivationMode::Hold => {
                if is_key_down(LEFT_CLICK_KEY) != mouse_state.0 {
                    toggle_mouse(&mouse, Keys::LEFT, mouse_state.0);
                    mouse_state.0 = !mouse_state.0;
                }
                if is_key_down(MIDDLE_CLICK_KEY) != mouse_state.1 {
                    toggle_mouse(&mouse, Keys::MIDDLE, mouse_state.1);
                    mouse_state.1 = !mouse_state.1;
                }
                if is_key_down(RIGHT_CLICK_KEY) != mouse_state.2 {
                    toggle_mouse(&mouse, Keys::RIGHT, mouse_state.2);
                    mouse_state.2 = !mouse_state.2;
                }
            }
            ActivationMode::Toggle => {
                if is_key_pressed(LEFT_CLICK_KEY) {
                    toggle_mouse(&mouse, Keys::LEFT, mouse_state.0);
                    mouse_state.0 = !mouse_state.0;
                }
                if is_key_pressed(MIDDLE_CLICK_KEY) {
                    toggle_mouse(&mouse, Keys::MIDDLE, mouse_state.1);
                    mouse_state.1 = !mouse_state.1;
                }
                if is_key_pressed(RIGHT_CLICK_KEY) {
                    toggle_mouse(&mouse, Keys::RIGHT, mouse_state.2);
                    mouse_state.2 = !mouse_state.2;
                }
            }
        }

        // Scroll
        if is_key_pressed(SCROLL_UP_KEY) {
            mouse.wheel(SCROLL_STEPS).unwrap();
        }
        if is_key_pressed(SCROLL_DOWN_KEY) {
            mouse.wheel(-SCROLL_STEPS).unwrap();
        }

        // Move
        if is_key_pressed(SPEED_UP_KEY) {
            speed += SPEED_STEP;
            if movement_mode == MovementMode::Linear {
                step_size_x = speed;
                step_size_y = speed;
            }
        }
        if is_key_pressed(SPEED_DOWN_KEY) {
            speed = (speed - SPEED_STEP).max(0.0);
            if movement_mode == MovementMode::Linear {
                step_size_x = speed;
                step_size_y = speed;
            }
        }
        if is_key_pressed(MOVEMENT_MODE_KEY) {
            movement_mode.next();
        }

        // X
        if is_key_down(MOVE_LEFT_KEY) || is_key_down(MOVE_RIGHT_KEY) {
            match movement_mode {
                MovementMode::Linear => {}
                MovementMode::Quadratic => {
                    step_size_x += DEFAULT_SPEED/10.0
                }
                MovementMode::Exponential => {
                    step_size_x *= 1.04
                }
                MovementMode::SuperExponential => {
                    step_size_x *= 1.2
                }
            }
            if is_key_down(MOVE_RIGHT_KEY) {
                move_mouse_by(&mouse, step_size_x.floor() as i32, 0);
            }
            if is_key_down(MOVE_LEFT_KEY) {
                move_mouse_by(&mouse, -step_size_x.floor() as i32, 0);
            }
        }
        else {
            step_size_x = speed
        }
        
        // Y
        if is_key_down(MOVE_UP_KEY) || is_key_down(MOVE_DOWN_KEY) {
            match movement_mode {
                MovementMode::Linear => {}
                MovementMode::Quadratic => {
                    step_size_y += DEFAULT_SPEED/10.0
                }
                MovementMode::Exponential => {
                    step_size_y *= 1.04
                }
                MovementMode::SuperExponential => {
                    step_size_y *= 1.2
                }
            }
            if is_key_down(MOVE_UP_KEY) {
                move_mouse_by(&mouse, 0, -step_size_y.floor() as i32);
            }
            if is_key_down(MOVE_DOWN_KEY) {
                move_mouse_by(&mouse, 0, step_size_y.floor() as i32);
            }
        }
        else {
            step_size_y = speed
        }
        
        // Save/Load position
        for (i, slot_key) in SLOT_KEYS.iter().enumerate() {
            if is_key_pressed(*slot_key) {
                if is_key_down(SAVE_KEY) {
                    let position = mouse.get_position().unwrap();
                    saved_positions[i] = (position.x, position.y)
                }
                else {
                    mouse.move_to(saved_positions[i].0, saved_positions[i].1).unwrap();
                }
            }
        }

        if is_key_pressed(RESET_KEY) {
            speed = DEFAULT_SPEED;
            movement_mode = MovementMode::Linear;
            if mouse_state.0 {mouse.release(&Keys::LEFT).unwrap();}
            if mouse_state.1 {mouse.release(&Keys::MIDDLE).unwrap();}
            if mouse_state.2 {mouse.release(&Keys::RIGHT).unwrap();}
            mouse_state = (false, false, false);

        }
        
        next_frame().await;
    }
}