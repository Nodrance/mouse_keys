// 
// If you're looking to change the default settings, scroll to the bottom
// 

use inputbot::KeybdKey;

#[derive(Debug)]
pub struct PositionSet {
    saved_positions: Vec<(i32, i32)>,
}

impl PositionSet {
    pub fn new(position_count: usize) -> Self {
        Self {
            saved_positions: vec![(0, 0); position_count]
        }
    }
    pub fn save_position(&mut self, index: usize, coords: (i32, i32)) {
        if index > self.saved_positions.len() {return}
        self.saved_positions[index] = coords
    }
    pub fn load_position(&self, index: usize) -> Option<(i32, i32)> {
        if index > self.saved_positions.len() {return None}
        Some(self.saved_positions[index])
    }
}

#[derive(Debug)]
pub struct ModeSet {
    modes: Vec<MouseMode>,
    mode_index: usize,
}

impl ModeSet {
    pub fn new(starting_modes: &Vec<MouseMode>) -> Self {
        Self { 
            modes: starting_modes.clone(),
            mode_index: 0
        }
    }
    pub fn increment_mode(&mut self) -> usize {
        self.mode_index = (self.mode_index + 1) % self.modes.len();
        self.mode_index
    }
    pub fn get_mode(&self) -> MouseMode {
        self.modes[self.mode_index].clone()
    }
    pub fn reset_modes(&mut self) {
        self.mode_index = 0;
        self.modes.iter_mut().for_each(
            |mode| {mode.reset_speed();}
        );
    }
    pub fn increase_speed(&mut self) {
        self.modes[self.mode_index].speed_up();
    }
    pub fn decrease_speed(&mut self) {
        self.modes[self.mode_index].speed_up();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MovementKeys {
    pub left: KeybdKey,
    pub right: KeybdKey,
    pub up: KeybdKey,
    pub down: KeybdKey,
}


#[derive(Debug, Copy, Clone)]
pub struct SpeedKeys {
    pub up: KeybdKey,
    pub down: KeybdKey,
}


#[derive(Debug, Copy, Clone)]
pub struct StepKeys {
    pub big: KeybdKey,
    pub small: KeybdKey,
}


#[derive(Debug, Copy, Clone)]
pub struct MouseKeys {
    pub left: KeybdKey,
    pub middle: KeybdKey,
    pub right: KeybdKey,
}


#[derive(Debug, Copy, Clone)]
pub struct ScrollKeys {
    pub up: KeybdKey,
    pub down: KeybdKey,
}

#[derive(Debug, Clone)]
pub struct ControllerKeys {
    pub activate: KeybdKey,
    pub mode: KeybdKey,
    pub reset: KeybdKey,
    pub movement: MovementKeys,
    pub speed: SpeedKeys,
    pub step: StepKeys,
    pub mouse: MouseKeys,
    pub scroll: ScrollKeys,
    pub save: KeybdKey,
    pub slots: Vec<KeybdKey>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActivationMode {
    Hold,
    Toggle
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccelerationType {
    Linear,
    Quadratic,
    Exponential
}

#[derive(Debug, Clone)]
pub struct MouseMode {
    pub acceleration_type: AccelerationType,
    pub default_speed: f32,
    pub current_speed: f32,
    pub step: f32,
    pub acceleration: f32
}

impl MouseMode {
    pub fn reset_speed(&mut self) {
        self.current_speed = self.default_speed
    }
    pub fn speed_up(&mut self) {
        self.current_speed += self.step
    }
    pub fn speed_down(&mut self) {
        self.current_speed = (self.current_speed - self.step).max(0.0);
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MouseActivationModes {
    pub activation: ActivationMode,
    pub left: ActivationMode,
    pub middle: ActivationMode,
    pub right: ActivationMode,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub toggles: MouseActivationModes,
    pub scroll: usize,
    pub modes: Vec<MouseMode>
}

#[derive(Debug, Clone)]
pub struct Settings { // For the stuff that's set when the program starts and doesn't change
    pub keys: ControllerKeys,
    pub config: Config
}
// fn new(filepath: &str) -> Settings {
//     let mut config_file = File::open(filepath);
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Deserializer::parse(&contents).unwrap().into()
// }

impl Default for Settings {
    fn default() -> Self {
        Settings { 
            keys: ControllerKeys {
                activate: KeybdKey::BackquoteKey,
                mode: KeybdKey::TabKey,
                reset: KeybdKey::BackspaceKey,
                movement: MovementKeys { 
                    left: KeybdKey::LeftKey,
                    right: KeybdKey::RightKey, 
                    up: KeybdKey::UpKey,
                    down: KeybdKey::DownKey,
                },
                speed: SpeedKeys {
                    up: KeybdKey::EqualKey,
                    down: KeybdKey::MinusKey,
                },
                step: StepKeys { 
                    big: KeybdKey::LControlKey, 
                    small: KeybdKey::LAltKey,
                },
                mouse: MouseKeys { 
                    left: KeybdKey::SpaceKey,
                    middle: KeybdKey::BackslashKey, 
                    right: KeybdKey::EnterKey,
                },
                scroll: ScrollKeys { 
                    up: KeybdKey::PeriodKey, 
                    down: KeybdKey::CommaKey,
                },
                save: KeybdKey::LShiftKey,
                slots: vec![
                    KeybdKey::Numrow1Key,
                    KeybdKey::Numrow2Key,
                    KeybdKey::Numrow3Key,
                    KeybdKey::Numrow4Key,
                    KeybdKey::Numrow5Key,
                    KeybdKey::Numrow6Key,
                    KeybdKey::Numrow7Key,
                    KeybdKey::Numrow8Key,
                    KeybdKey::Numrow9Key,
                    KeybdKey::Numrow0Key,
                ]
            },
            config: Config { 
                toggles: MouseActivationModes { 
                    activation: ActivationMode::Toggle, 
                    left: ActivationMode::Hold, 
                    middle: ActivationMode::Hold, 
                    right: ActivationMode::Hold,
                },
                scroll: 2,
                modes: vec![
                    MouseMode {
                        acceleration_type: AccelerationType::Linear,
                        default_speed: 10.0,
                        current_speed: 0.0, // no effect here, used to keep track after you raise or lower the speed
                        step: 2.5,
                        acceleration: 0.0
                    },
                    MouseMode {
                        acceleration_type: AccelerationType::Quadratic,
                        default_speed: 4.0,
                        current_speed: 0.0,
                        step: 1.0,
                        acceleration: 0.5
                    },
                    MouseMode {
                        acceleration_type: AccelerationType::Exponential,
                        default_speed: 1.0,
                        current_speed: 0.0,
                        step: 0.25,
                        acceleration: 1.08
                    },
                    MouseMode {
                        acceleration_type: AccelerationType::Exponential,
                        default_speed: 1.0,
                        current_speed: 0.0,
                        step: 1.25,
                        acceleration: 1.2
                    }
                ]
            }
        }
    }
}