use inputbot::{BlockInput, MouseButton, MouseCursor, MouseWheel};
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
    let toggle = mouse_settings.config.toggles.activation;
    mouse_settings.keybinds.activate.block_bind(move || {
        match toggle {
            ActivationMode::Toggle => {
                let is_enabled = !enabled_copy.fetch_not(Ordering::AcqRel);
                if !is_enabled {
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
                match is_enabled {
                    true => {println!("Mouse Keys enabled");}
                    false => {println!("Mouse Keys disabled");}
                }
            }
            ActivationMode::Hold => {
                enabled_copy.store(true, Ordering::Relaxed);
            }
        }
    });
    if toggle == ActivationMode::Hold {
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        mouse_settings.keybinds.activate.bind_release(move || {
            enabled_copy.store(false, Ordering::Relaxed);
        });
    }

    // Change Mode

    let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
    let modes_copy = Arc::clone(&mode_set);
    if let Some(keybind) = mouse_settings.keybinds.mode {
        keybind.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                let mode_index = modes_copy.lock().unwrap().increment_mode();
                println!("Now using mode #{}", mode_index + 1);
                BlockInput::Block
            } else {
                BlockInput::DontBlock
            }
        });
    }

    // Reset

    let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
    let modes_copy = Arc::clone(&mode_set);
    if let Some(keybind) = mouse_settings.keybinds.reset {
        keybind.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                modes_copy.lock().unwrap().reset_modes();
                println!("Reset speeds and changed to mode #1");
                BlockInput::Block
            } else {
                BlockInput::DontBlock
            }
        });
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

    for (pressed, keybind, direction) in [
        (is_up_pressed, mouse_settings.keybinds.movement.up, Direction::Up),
        (is_down_pressed, mouse_settings.keybinds.movement.down, Direction::Down),
        (is_left_pressed, mouse_settings.keybinds.movement.left, Direction::Left),
        (is_right_pressed, mouse_settings.keybinds.movement.right, Direction::Right),
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
                    let (mut dx, mut dy) = movement.get_delta();
                    if let Some(step_keybind) = mouse_settings.keybinds.step.big &&
                      step_keybind.is_pressed() {
                        dx *= 2.0;
                        dy *= 2.0;
                    };
                    if let Some(step_keybind) = mouse_settings.keybinds.step.small &&
                      step_keybind.is_pressed() {
                        dx /= 2.0;
                        dy /= 2.0;
                    };
                    offset_copy.0.fetch_add(dx as isize, Ordering::Relaxed);
                    offset_copy.1.fetch_add(dy as isize, Ordering::Relaxed);
                    thread::sleep(Duration::from_millis(FRAMETIME));
                }
            }
        });
        if let Some(key) = keybind {
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

    // Change speed

    for (keybind, increase) in [
        (mouse_settings.keybinds.speed.up, true),
        (mouse_settings.keybinds.speed.down, false),
    ] {
        if let Some(key) = keybind {
            let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
            let modes_copy = Arc::clone(&mode_set);
            key.blockable_bind(move || {
                if enabled_copy.load(Ordering::Relaxed) {
                    let big_step = {
                        if let Some(step_keybind) = mouse_settings.keybinds.step.big {
                            step_keybind.is_pressed()
                        } else {false}
                    };
                    let small_step = {
                        if let Some(step_keybind) = mouse_settings.keybinds.step.small {
                            step_keybind.is_pressed()
                        } else {false}
                    };
                    modes_copy.lock().unwrap().change_speed(increase, big_step, small_step);
                    BlockInput::Block
                } else {
                    BlockInput::DontBlock
                }
            });
        }
    }

    // Mouse Buttons

    for (keybind, mouse_button, toggle) in [
        (mouse_settings.keybinds.mouse.left, MouseButton::LeftButton, mouse_settings.config.toggles.left),
        (mouse_settings.keybinds.mouse.middle, MouseButton::MiddleButton, mouse_settings.config.toggles.middle),
        (mouse_settings.keybinds.mouse.right, MouseButton::RightButton, mouse_settings.config.toggles.right),
    ] {
        if let Some(key) = keybind {
            let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
            key.blockable_bind(move || {
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
            if toggle == ActivationMode::Hold {
                let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
                key.bind_release(move || {
                    if enabled_copy.load(Ordering::Relaxed)
                    && mouse_button.is_pressed() {
                        mouse_button.release();
                    }
                });
            }
        }
    }

    // Scroll

    for (keybind, direction) in [
        (mouse_settings.keybinds.scroll.up, Direction::Up),
        (mouse_settings.keybinds.scroll.down, Direction::Down),
        (mouse_settings.keybinds.scroll.left, Direction::Left),
        (mouse_settings.keybinds.scroll.right, Direction::Right),

    ] {
        if let Some(key) = keybind {
            let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
            key.blockable_bind(move || {
                if enabled_copy.load(Ordering::Relaxed) {
                    let scroll = mouse_settings.config.scroll;
                    match direction {
                        Direction::Up => {MouseWheel::scroll_ver(scroll as i32);}
                        Direction::Down => {MouseWheel::scroll_ver(-(scroll as i32));}
                        Direction::Left => {MouseWheel::scroll_hor(-(scroll as i32));}
                        Direction::Right => {MouseWheel::scroll_hor(scroll as i32);}
                    }
                    BlockInput::Block
                } else {
                    BlockInput::DontBlock
                }
            });
        }
    }

    // Saving

    let position_set = Arc::new(Mutex::new(
        PositionSet::new(mouse_settings.keybinds.slots.len())
    ));
    for (index, keybind) in mouse_settings.keybinds.slots.iter().enumerate() {
        let enabled_copy = Arc::clone(&is_mouse_keys_enabled);
        let position_copy = Arc::clone(&position_set);
        keybind.blockable_bind(move || {
            if enabled_copy.load(Ordering::Relaxed) {
                if mouse_settings.keybinds.save.is_pressed() {
                    let (x,y) = MouseCursor::pos();
                    position_copy.lock().unwrap().save_position(index, (x,y));
                } else {
                    let (x,y) = position_copy.lock().unwrap().load_position(index).unwrap_or((0,0));
                    MouseCursor::move_abs(x, y);
                }
                BlockInput::Block
            } else {
                BlockInput::DontBlock
            }
        });
    }

    inputbot::handle_input_events(false);
}