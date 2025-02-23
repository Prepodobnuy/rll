use std::sync::mpsc::{self, Receiver};
use crate::layout::{Container, StyleLink};

pub trait InputListenerTrait {
}

pub struct Key {
    pub keycode: KeyCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
}

pub enum InputAction {
    Press,
    Hold,
    Release,
}

pub struct Input {
    pub key_code: KeyCode,
    pub action: InputAction,
}

impl Input {
    pub fn new(key_code: KeyCode, action: InputAction) -> Self {
        Input {
            key_code,
            action,
        }
    }
}