use display_info::DisplayInfo;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub capitalization: Capitalization,
    pub follow_for_caps_sensitive: bool,
    pub follow_for_caps_insensitive: bool,
    pub category: String,
    pub keyboard: usize,
    pub style: usize,
    pub mouse_from_center: bool,
    pub mouse_sensitivity: f32,
    pub min_press_time: u128,
    pub scroll_hold_time: u64,
    pub window_title: String,
    pub display_choice: DisplayChoice,
    pub update_text_position: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let displays = DisplayInfo::all().unwrap();

        let display_id = displays
            .iter()
            .find(|d| d.is_primary)
            .unwrap_or(&displays[0])
            .id;

        Self {
            capitalization: Capitalization::Follow,
            follow_for_caps_sensitive: false,
            follow_for_caps_insensitive: false,
            category: String::new(),
            keyboard: 0,
            style: 0,
            mouse_from_center: false,
            mouse_sensitivity: 50.0,
            min_press_time: 0,
            scroll_hold_time: 100,
            window_title: "NuhxBoard".into(),
            display_choice: DisplayChoice {
                id: display_id,
                primary: true,
            },
            update_text_position: true,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Capitalization {
    Lower,
    Upper,
    Follow,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct DisplayChoice {
    pub id: u32,
    pub primary: bool,
}

impl Display for DisplayChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        if self.primary {
            write!(f, "(primary)")?;
        }
        Ok(())
    }
}
