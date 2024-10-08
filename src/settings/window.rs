use wgpu::{PowerPreference, PresentMode};

pub struct WindowSettings {
    pub power_preference: PowerPreference,
    pub present_mode: PresentMode,
    pub desired_max_buffer: u32,
}

pub static WINDOW_SETTINGS: WindowSettings = WindowSettings {
    power_preference: PowerPreference::None,
    present_mode: PresentMode::Immediate,
    desired_max_buffer: 2,
};