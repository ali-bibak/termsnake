use ansi_escapes::*;

pub struct Consts {}

impl Consts {
    /// window
    pub const WINDOW_H: usize = 23;
    pub const WINDOW_W: usize = 80;

    /// terminal manipulation
    pub const ESC: char = 27u8 as char;
    pub const BACKSPACE: char = 8u8 as char;

    pub fn up() -> String {
        format!("{}[A", Consts::ESC)
    }

    pub fn erase() -> String {
        format!("{}[2K", Consts::ESC)
    }

    pub fn clear_screen() -> String {
        format!("{}[2J", Consts::ESC)
    }

    pub fn clean() -> String {
        let mut s = "".to_string();
        s.push_str(&Consts::erase());
        for _ in 0..Consts::WINDOW_H {
            s.push_str(&Consts::up());
        }
        for _ in 0..Consts::WINDOW_W {
            s.push_str(&Consts::erase());
        }
        s.push('\r');
        return s;
    }
}