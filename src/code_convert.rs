use rdev::{Button, Key};

pub fn mouse_button_code_convert(rdev_button: Button) -> u32 {
    match rdev_button {
        Button::Left => 0,
        Button::Middle => 2,
        Button::Right => 1,
        Button::Unknown(code) => match code {
            8 => 3,
            9 => 4,
            _ => panic!("Unknown mouse button code: {:?}", rdev_button),
        },
    }
}

pub fn keycode_convert(rdev_key: Key) -> u32 {
    match rdev_key {
        Key::Backspace => 8,
        Key::Tab => 9,
        Key::Return => 13,
        Key::Pause => 19,
        Key::CapsLock => 20,
        Key::Escape => 27,
        Key::Space => 32,
        Key::PageUp => 33,
        Key::PageDown => 34,
        Key::End => 35,
        Key::Home => 36,
        Key::LeftArrow => 37,
        Key::UpArrow => 38,
        Key::RightArrow => 39,
        Key::DownArrow => 40,
        Key::PrintScreen => 44,
        Key::Insert => 45,
        Key::Delete => 46,
        Key::Num0 => 48,
        Key::Num1 => 49,
        Key::Num2 => 50,
        Key::Num3 => 51,
        Key::Num4 => 52,
        Key::Num5 => 53,
        Key::Num6 => 54,
        Key::Num7 => 55,
        Key::Num8 => 56,
        Key::Num9 => 57,
        Key::KeyA => 65,
        Key::KeyB => 66,
        Key::KeyC => 67,
        Key::KeyD => 68,
        Key::KeyE => 69,
        Key::KeyF => 70,
        Key::KeyG => 71,
        Key::KeyH => 72,
        Key::KeyI => 73,
        Key::KeyJ => 74,
        Key::KeyK => 75,
        Key::KeyL => 76,
        Key::KeyM => 77,
        Key::KeyN => 78,
        Key::KeyO => 79,
        Key::KeyP => 80,
        Key::KeyQ => 81,
        Key::KeyR => 82,
        Key::KeyS => 83,
        Key::KeyT => 84,
        Key::KeyU => 85,
        Key::KeyV => 86,
        Key::KeyW => 87,
        Key::KeyX => 88,
        Key::KeyY => 89,
        Key::KeyZ => 90,
        Key::MetaLeft => 91,
        Key::MetaRight => 92,
        Key::Kp0 => 96,
        Key::Kp1 => 97,
        Key::Kp2 => 98,
        Key::Kp3 => 99,
        Key::Kp4 => 100,
        Key::Kp5 => 101,
        Key::Kp6 => 102,
        Key::Kp7 => 103,
        Key::Kp8 => 104,
        Key::Kp9 => 105,
        Key::KpMultiply => 106,
        Key::KpPlus => 107,
        Key::KpMinus => 109,
        Key::KpDelete => 110,
        Key::KpDivide => 111,
        Key::F1 => 112,
        Key::F2 => 113,
        Key::F3 => 114,
        Key::F4 => 115,
        Key::F5 => 116,
        Key::F6 => 117,
        Key::F7 => 118,
        Key::F8 => 119,
        Key::F9 => 120,
        Key::F10 => 121,
        Key::F11 => 122,
        Key::F12 => 123,
        Key::ScrollLock => 145,
        Key::ShiftLeft => 160,
        Key::ShiftRight => 161,
        Key::ControlLeft => 162,
        Key::ControlRight => 163,
        Key::Alt => 164,
        Key::AltGr => 165,
        Key::SemiColon => 186,
        Key::Equal => 187,
        Key::Comma => 188,
        Key::Minus => 189,
        Key::Dot => 190,
        Key::Slash => 191,
        Key::BackQuote => 192,
        Key::LeftBracket => 219,
        Key::BackSlash => 220,
        Key::RightBracket => 221,
        Key::Quote => 222,
        Key::NumLock => 144,
        Key::KpReturn => 1025,
        // Menu
        Key::Unknown(135) => 93,
        _ => panic!("Unknown xinput code: {:?}", rdev_key),
    }
}
