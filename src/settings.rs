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
    pub fn new(starting_modes: &[MouseMode]) -> Self {
        Self {
            modes: starting_modes.to_owned(),
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
    pub fn change_speed(&mut self, increase: bool, big_step: bool, small_step: bool) {
        match increase {
            true => {self.increase_speed(big_step, small_step);},
            false => {self.decrease_speed(big_step, small_step);}
        }
    }
    pub fn increase_speed(&mut self, big_step: bool, small_step: bool) {
        self.modes[self.mode_index].speed_up(big_step, small_step);
    }
    pub fn decrease_speed(&mut self, big_step: bool, small_step: bool) {
        self.modes[self.mode_index].speed_down(big_step, small_step);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MovementKeys {
    pub up: Option<KeybdKey>,
    pub down: Option<KeybdKey>,
    pub left: Option<KeybdKey>,
    pub right: Option<KeybdKey>,
}


#[derive(Debug, Copy, Clone)]
pub struct SpeedKeys {
    pub up: Option<KeybdKey>,
    pub down: Option<KeybdKey>,
}


#[derive(Debug, Copy, Clone)]
pub struct StepKeys {
    pub big: Option<KeybdKey>,
    pub small: Option<KeybdKey>,
}


#[derive(Debug, Copy, Clone)]
pub struct MouseKeys {
    pub left: Option<KeybdKey>,
    pub middle: Option<KeybdKey>,
    pub right: Option<KeybdKey>,
}


#[derive(Debug, Copy, Clone)]
pub struct ScrollKeys {
    pub up: Option<KeybdKey>,
    pub down: Option<KeybdKey>,
    pub left: Option<KeybdKey>,
    pub right: Option<KeybdKey>,
}

#[derive(Debug, Clone)]
pub struct ControllerKeys {
    pub activate: KeybdKey,
    pub mode: Option<KeybdKey>,
    pub reset: Option<KeybdKey>,
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
    LinearStep{step_time: usize},
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
    pub fn new(
        acceleration_type: AccelerationType,
        default_speed: f32,
        step: f32,
        acceleration: f32
    ) -> Self {
        Self {acceleration_type, default_speed, current_speed: default_speed, step, acceleration}
    }
    pub fn reset_speed(&mut self) {
        self.current_speed = self.default_speed
    }
    pub fn speed_up(&mut self, is_big_step: bool, is_small_step: bool) {
        let mut step = self.step;
        if is_big_step {step *= 4.0}
        if is_small_step {step /= 4.0}
        self.current_speed += step;
    }
    pub fn speed_down(&mut self, is_big_step: bool, is_small_step: bool) {
        let mut step = self.step;
        if is_big_step {step *= 4.0}
        if is_small_step {step /= 4.0}
        self.current_speed = (self.current_speed - step).max(0.0);
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
    pub keybinds: ControllerKeys,
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
            keybinds: ControllerKeys { 
                // available keys: https://github.com/obv-mikhail/InputBot/blob/develop/src/public.rs#L97
                // For most keys you need Some(KeybdKey:: name ) 
                // If you want to disable a key, set it to None
                // A few keys (like activating) don't have Some and can't be set to None
                activate: KeybdKey::BackquoteKey, //Activates the program. This is ` (or ~), next to 1 and above tab on my keyboard
                mode: Some(KeybdKey::TabKey), // Changes to the next mode (you can add your own modes below)
                reset: Some(KeybdKey::BackspaceKey), // Resets speed and mode
                movement: MovementKeys {
                    left: Some(KeybdKey::LeftKey), // Left arrow
                    right: Some(KeybdKey::RightKey),
                    up: Some(KeybdKey::UpKey),
                    down: Some(KeybdKey::DownKey),
                },
                speed: SpeedKeys {
                    up: Some(KeybdKey::EqualKey), // +
                    down: Some(KeybdKey::MinusKey),
                },
                step: StepKeys {
                    big: Some(KeybdKey::LControlKey), // While held, makes moving and speed changes be 4x
                    small: Some(KeybdKey::LAltKey), // Same but divided
                },
                mouse: MouseKeys {
                    left: Some(KeybdKey::SpaceKey),
                    middle: Some(KeybdKey::BackslashKey), // Chosen because it's above enter on my current layout
                    right: Some(KeybdKey::EnterKey),
                },
                scroll: ScrollKeys {
                    up: Some(KeybdKey::PeriodKey), // >
                    down: Some(KeybdKey::CommaKey), // <
                    left: None, // Feel free to set these to Some(thing)
                    right: None, 
                },
                save: KeybdKey::LShiftKey, // Hold this to save instead of load
                slots: vec![
                    KeybdKey::Numrow1Key, // press this to load
                    KeybdKey::Numrow2Key, 
                    KeybdKey::Numrow3Key, 
                    KeybdKey::Numrow4Key,
                    KeybdKey::Numrow5Key,
                    KeybdKey::Numrow6Key,
                    KeybdKey::Numrow7Key, // add or remove keys to add or remove slots
                    KeybdKey::Numrow8Key, // order doesn't matter
                    KeybdKey::Numrow9Key,
                    KeybdKey::Numrow0Key,
                ]
            },
            config: Config {
                toggles: MouseActivationModes {
                    activation: ActivationMode::Toggle, // Available modes are Toggle and Hold
                    left: ActivationMode::Hold, // You can set it per individual mouse key
                    middle: ActivationMode::Hold,
                    right: ActivationMode::Hold,
                },
                scroll: 2, // How many lines to scroll by when you press the scroll keys
                modes: vec![
                    MouseMode::new(
                        AccelerationType::Linear,
                        10.0, // starting speed
                        2.5,  // how much to change the speed by when you change it
                        0.0   // acceleration, no effect for linear
                    ),
                    MouseMode::new(
                        AccelerationType::LinearStep{step_time: 45}, // starts linear, then after 60 frames multiplies speed by acceleration
                        4.0, // start at 4
                        1.0, // go up or down by 1 when you press the change speed button
                        5.0 // go to 4*5=20 after 60 frames
                    ),
                    MouseMode::new(
                        AccelerationType::Quadratic, // adds acceleration to speed every frame
                        4.0, // start at 4
                        1.0, // speed is saved per mode
                        0.5 // then 4.5, then 5.0, then 5.5
                    ),
                    MouseMode::new(
                        AccelerationType::Exponential, // multiplies speed by acceleration every frame
                        1.0, //start at 1
                        0.25, 
                        1.1 // exponentials grow FAST
                    )
                ]
            }
        }
    }
}