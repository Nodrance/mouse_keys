use inputbot::{BlockInput, MouseButton};
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

mod settings;
mod motion;
use crate::{settings::*, motion::*};

fn toggle_button(button: MouseButton) {
    if button.is_pressed() {
        button.release();
    } else {
        button.press();
    }
}

const FRAMETIME: u64 = 1000/60; // 1/60th of a second, in milliseconds

fn main() {
    let mouse_settings = Settings::default(); // To change the settings including keybinds and default speeds/modes, edit the bottom of settings.rs

    let current_enabled = 
    Arc::new(AtomicBool::new(false));

    let is_up_pressed = 
    Arc::new(AtomicBool::new(false));
    let is_down_pressed = 
    Arc::new(AtomicBool::new(false));
    let is_left_pressed = 
    Arc::new(AtomicBool::new(false));
    let is_right_pressed = 
    Arc::new(AtomicBool::new(false));
    
    let current_modes = Arc::new(Mutex::new(
        ModeSet::new(
        &mouse_settings.config.modes
    )));
    
    // Mouse Buttons

    let enabled_copy = Arc::clone(&current_enabled);
    mouse_settings.keys.mouse.left.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            toggle_button(MouseButton::LeftButton);
            BlockInput::Block
        } else {
            println!("Unclicked");
            BlockInput::DontBlock
        }
    });

    let enabled_copy = Arc::clone(&current_enabled);
    mouse_settings.keys.mouse.middle.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            toggle_button(MouseButton::MiddleButton);
            BlockInput::Block
        } else {
            println!("Unclicked");
            BlockInput::DontBlock
        }
    });

    let enabled_copy = Arc::clone(&current_enabled);
    mouse_settings.keys.mouse.right.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            toggle_button(MouseButton::RightButton);
            BlockInput::Block
        } else {
            println!("Unclicked");
            BlockInput::DontBlock
        }
    });

    // Change Mode

    let enabled_copy = Arc::clone(&current_enabled);
    let modes_copy = Arc::clone(&current_modes);
    mouse_settings.keys.mode.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mode_index = modes_copy.lock().unwrap().increment_mode();
            println!("Now using mode #{}", mode_index + 1);
            BlockInput::Block
        } else {
            BlockInput::DontBlock
        }
    });

    // Movement
    for (pressed, key, direction) in [
        (is_up_pressed, mouse_settings.keys.movement.up, Direction::Up),
        (is_down_pressed, mouse_settings.keys.movement.down, Direction::Down),
        (is_left_pressed, mouse_settings.keys.movement.left, Direction::Left),
        (is_right_pressed, mouse_settings.keys.movement.right, Direction::Right),
    ] {
        let enabled_copy = Arc::clone(&current_enabled);
        let modes_copy = Arc::clone(&current_modes);
        let pressed_copy = Arc::clone(&pressed);
        key.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                let mut movement = MouseMotion::new(
                        modes_copy.lock().unwrap().get_mode(),
                        direction);
                let enabled_copy_copy = Arc::clone(&enabled_copy);
                let pressed_copy_copy = Arc::clone(&pressed_copy);
                thread::spawn(move || {
                    let swap_result = pressed_copy_copy.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
                    if swap_result.is_err() {return}
                    while
                        pressed_copy_copy.load(Ordering::Relaxed) &&
                        enabled_copy_copy.load(Ordering::Relaxed) {
                        movement.increment_speed();
                        let (dx, dy) = movement.get_delta();
                        inputbot::MouseCursor::move_rel(dx, dy);
                        thread::sleep(Duration::from_millis(FRAMETIME));
                    }
                });
                BlockInput::Block
            } else {
                BlockInput::DontBlock
            }
        });
        let pressed_copy = Arc::clone(&pressed);
        key.bind_release( move || {
            pressed_copy.store(false, Ordering::SeqCst);
        });
    }

    // Activation

    let enabled_copy = Arc::clone(&current_enabled);
    mouse_settings.keys.activate.block_bind( move || {
        enabled_copy.fetch_not(Ordering::AcqRel);
        if !enabled_copy.load(Ordering::Acquire) {
            if MouseButton::LeftButton.is_pressed() {
                MouseButton::LeftButton.release();
            }
            if MouseButton::MiddleButton.is_pressed() {
                MouseButton::MiddleButton.release();
            }
            if MouseButton::RightButton.is_pressed() {
                MouseButton::RightButton.release();
            }
        }
        match enabled_copy.load(Ordering::Acquire) {
            true => {println!("Mouse Keys enabled");}
            false => {println!("Mouse Keys disabled");}
        }
    });
    
    inputbot::handle_input_events(false);
}