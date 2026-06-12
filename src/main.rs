use inputbot::{BlockInput, MouseButton, MouseCursor};
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicIsize, Ordering}};

mod settings;
mod motion;
use crate::{settings::*, motion::*};

const FRAMETIME: u64 = 1000/60; // 1/60th of a second, in milliseconds

fn main() {
    let mouse_settings = Settings::default(); // To change the settings including keybinds and default speeds/modes, edit the bottom of settings.rs
    let is_mouse_keys_enabled = Arc::new(AtomicBool::new(false));
    let mode_set = Arc::new(Mutex::new(
        ModeSet::new(&mouse_settings.config.modes)));

    
    // Activation

    let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
    mouse_settings.keys.activate.block_bind( move || {
        enabled_copy.fetch_not(Ordering::AcqRel);
        if !enabled_copy.load(Ordering::Acquire) {
            for button in [
                MouseButton::LeftButton,
                MouseButton::MiddleButton,
                MouseButton::RightButton
            ] {
                if button.is_pressed() {
                    button.release();
                }
            }
        }
        match enabled_copy.load(Ordering::Acquire) {
            true => {println!("Mouse Keys enabled");}
            false => {println!("Mouse Keys disabled");}
        }
    });

    
    // Change Mode

    let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
    let modes_copy = Arc::clone(&mode_set);
    mouse_settings.keys.mode.blockable_bind(move || {
        if enabled_copy.load(Ordering::Relaxed) {
            let mode_index = modes_copy.lock().unwrap().increment_mode();
            println!("Now using mode #{}", mode_index + 1);
            BlockInput::Block
        } else {
            BlockInput::DontBlock
        }
    });
    
    // Mouse Buttons

    for (keybind, mouse_button, toggle) in [
        (mouse_settings.keys.mouse.left, MouseButton::LeftButton, mouse_settings.config.toggles.left),
        (mouse_settings.keys.mouse.middle, MouseButton::MiddleButton, mouse_settings.config.toggles.middle),
        (mouse_settings.keys.mouse.right, MouseButton::RightButton, mouse_settings.config.toggles.right),
    ] {
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        keybind.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                match toggle {
                    ActivationMode::Toggle => {
                        if mouse_button.is_pressed() {
                            mouse_button.release();
                        } else {
                            mouse_button.press();
                        }
                    }
                    ActivationMode::Hold => {
                        if !mouse_button.is_pressed() {
                            mouse_button.press();
                        }
                    }
                }
                BlockInput::Block
            } else {
                BlockInput::DontBlock
            }
        });
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        if toggle == ActivationMode::Hold {
            keybind.bind_release(move || {
                if enabled_copy.load(Ordering::Relaxed)
                && mouse_button.is_pressed() {
                    mouse_button.release();
                }
            });
        }
    }

    // Movement calculation

    let is_up_pressed = 
        Arc::new(AtomicBool::new(false));
    let is_down_pressed = 
        Arc::new(AtomicBool::new(false));
    let is_left_pressed = 
        Arc::new(AtomicBool::new(false));
    let is_right_pressed = 
        Arc::new(AtomicBool::new(false));
    let current_remaining_offset = Arc::new(
        (AtomicIsize::from(0), AtomicIsize::from(0))
    );

    for (pressed, key, direction) in [
        (is_up_pressed, mouse_settings.keys.movement.up, Direction::Up),
        (is_down_pressed, mouse_settings.keys.movement.down, Direction::Down),
        (is_left_pressed, mouse_settings.keys.movement.left, Direction::Left),
        (is_right_pressed, mouse_settings.keys.movement.right, Direction::Right),
    ] {
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        let modes_copy = Arc::clone(&mode_set);
        let pressed_copy = Arc::clone(&pressed);
        let offset_copy = Arc::clone(&current_remaining_offset);
        thread::spawn(move || {
            loop {
                while !(pressed_copy.load(Ordering::Relaxed) && 
                        enabled_copy.load(Ordering::Relaxed)) {
                    thread::sleep(Duration::from_millis(FRAMETIME));
                }
                let mut movement = MouseMotion::new(
                            modes_copy.lock().unwrap().get_mode(),
                            direction
                        );
                while pressed_copy.load(Ordering::Relaxed) && 
                      enabled_copy.load(Ordering::Relaxed) {
                    movement.increment_speed();
                    let (dx, dy) = movement.get_delta();
                    offset_copy.0.fetch_add(dx, Ordering::Relaxed);
                    offset_copy.1.fetch_add(dy, Ordering::Relaxed);
                    thread::sleep(Duration::from_millis(FRAMETIME));
                }
            }
        });
        let pressed_copy = Arc::clone(&pressed);
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        key.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                pressed_copy.store(true, Ordering::SeqCst);
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

    // Actually move

    let offset_copy = Arc::clone(&current_remaining_offset);
    thread::spawn(move || {
        loop {
            let dx = offset_copy.0.swap(0, Ordering::Relaxed);
            let dy = offset_copy.1.swap(0, Ordering::Relaxed);
            MouseCursor::move_rel(
                dx.try_into().unwrap_or_default(),
                dy.try_into().unwrap_or_default(),
            );
            thread::sleep(Duration::from_millis(FRAMETIME));
        }
    });
    
    inputbot::handle_input_events(false);
}