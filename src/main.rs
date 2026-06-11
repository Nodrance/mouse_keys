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

    let enabled_copy = Arc::clone(&current_enabled);
    let modes_copy = Arc::clone(&current_modes);
    let up_copy = Arc::clone(&is_up_pressed);
    mouse_settings.keys.movement.up.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mut movement = MouseMotion::new(
                    modes_copy.lock().unwrap().get_mode(),
                    Direction::Up);
            let enabled_copy_copy = Arc::clone(&enabled_copy);
            let up_copy_copy = Arc::clone(&up_copy);
            thread::spawn(move || {
                let swap_result = up_copy_copy.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
                if swap_result.is_err() {return}
                while
                    up_copy_copy.load(Ordering::Relaxed) &&
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
    let up_copy = Arc::clone(&is_up_pressed);
    mouse_settings.keys.movement.up.bind_release( move || {
        up_copy.store(false, Ordering::SeqCst);
    });

    let enabled_copy = Arc::clone(&current_enabled);
    let modes_copy = Arc::clone(&current_modes);
    let down_copy = Arc::clone(&is_down_pressed);
    mouse_settings.keys.movement.down.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mut movement = MouseMotion::new(
                    modes_copy.lock().unwrap().get_mode(),
                    Direction::Down);
            let enabled_copy_copy = Arc::clone(&enabled_copy);
            let down_copy_copy = Arc::clone(&down_copy);
            thread::spawn(move || {
                let swap_result = down_copy_copy.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
                if swap_result.is_err() {return}
                while
                    down_copy_copy.load(Ordering::Relaxed) &&
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
    let down_copy = Arc::clone(&is_down_pressed);
    mouse_settings.keys.movement.down.bind_release( move || {
        down_copy.store(false, Ordering::SeqCst);
    });

    let enabled_copy = Arc::clone(&current_enabled);
    let modes_copy = Arc::clone(&current_modes);
    let left_copy = Arc::clone(&is_left_pressed);
    mouse_settings.keys.movement.left.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mut movement = MouseMotion::new(
                    modes_copy.lock().unwrap().get_mode(),
                    Direction::Left);
            let enabled_copy_copy = Arc::clone(&enabled_copy);
            let left_copy_copy = Arc::clone(&left_copy);
            thread::spawn(move || {
                let swap_result = left_copy_copy.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
                if swap_result.is_err() {return}
                while
                    left_copy_copy.load(Ordering::Relaxed) &&
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
    let left_copy = Arc::clone(&is_left_pressed);
    mouse_settings.keys.movement.left.bind_release( move || {
        left_copy.store(false, Ordering::SeqCst);
    });

    let enabled_copy = Arc::clone(&current_enabled);
    let modes_copy = Arc::clone(&current_modes);
    let right_copy = Arc::clone(&is_right_pressed);
    mouse_settings.keys.movement.right.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mut movement = MouseMotion::new(
                    modes_copy.lock().unwrap().get_mode(),
                    Direction::Right);
            let enabled_copy_copy = Arc::clone(&enabled_copy);
            let right_copy_copy = Arc::clone(&right_copy);
            thread::spawn(move || {
                let swap_result = right_copy_copy.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
                if swap_result.is_err() {return}
                while
                    right_copy_copy.load(Ordering::Relaxed) &&
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
    let right_copy = Arc::clone(&is_right_pressed);
    mouse_settings.keys.movement.right.bind_release( move || {
        right_copy.store(false, Ordering::SeqCst);
    });

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