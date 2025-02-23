use std::{collections::HashMap, sync::mpsc::{channel, Receiver, Sender}, thread};

use crate::backends::{backend::{Input, InputAction, InputListenerTrait, Key, KeyCode}, traits::ListenerTrait};

pub struct Listener {
    press_callbacks: HashMap<KeyCode, Box<dyn FnMut()>>,
    hold_callbacks: HashMap<KeyCode, Box<dyn FnMut()>>,
    release_callbacks: HashMap<KeyCode, Box<dyn FnMut()>>,
    event_sender: Sender<Input>,
}

impl Listener {
    pub fn new() -> (Self, Receiver<Input>) {
        let (tx, rx) = channel();

        let listener = Listener {
            press_callbacks: HashMap::new(),
            hold_callbacks: HashMap::new(),
            release_callbacks: HashMap::new(),
            event_sender: tx,
        };

        (listener, rx)
    }

    pub fn on_press<F>(&mut self, key_code: KeyCode, func: F)
    where
        F: FnMut() + 'static,
    {
        self.press_callbacks.insert(key_code, Box::new(func));
    }

    pub fn on_hold<F>(&mut self, key_code: KeyCode, func: F)
    where
        F: FnMut() + 'static,
    {
        self.hold_callbacks.insert(key_code, Box::new(func));
    }

    pub fn on_release<F>(&mut self, key_code: KeyCode, func: F)
    where
        F: FnMut() + 'static,
    {
        self.release_callbacks.insert(key_code, Box::new(func));
    }

    pub fn trigger(&mut self, input: Input) {
        if let Some(callback) = self.press_callbacks.get_mut(&input.key_code) {
            callback();
        }
    }
}

impl ListenerTrait for Listener {
    fn watch_input(&self) {
        let tx = self.event_sender.clone();
        thread::spawn(move || {
            loop {
                if let Ok(event) = crossterm::event::read() {
                    if let crossterm::event::Event::Key(key_event) = event {
                        let input = key_event_to_input(key_event);
                        tx.send(input).unwrap();
                    }
                }
            }
        });
    }
}

fn key_event_to_input(event: crossterm::event::KeyEvent) -> Input {
    let key_code = match event.code {
        crossterm::event::KeyCode::Backspace => {KeyCode::Backspace},
        crossterm::event::KeyCode::Enter => {KeyCode::Enter},
        crossterm::event::KeyCode::Left => {KeyCode::Left},
        crossterm::event::KeyCode::Right => {KeyCode::Right},
        crossterm::event::KeyCode::Up => {KeyCode::Up},
        crossterm::event::KeyCode::Down => {KeyCode::Down},
        crossterm::event::KeyCode::Home => {KeyCode::Home},
        crossterm::event::KeyCode::End => {KeyCode::End},
        crossterm::event::KeyCode::PageUp => {KeyCode::PageUp},
        crossterm::event::KeyCode::PageDown => {KeyCode::PageDown},
        crossterm::event::KeyCode::Tab => {KeyCode::Tab},
        crossterm::event::KeyCode::BackTab => {KeyCode::BackTab},
        crossterm::event::KeyCode::Delete => {KeyCode::Delete},
        crossterm::event::KeyCode::Insert => {KeyCode::Insert},
        crossterm::event::KeyCode::F(u8) => {KeyCode::F(u8)},
        crossterm::event::KeyCode::Char(char) => {KeyCode::Char(char)},
        crossterm::event::KeyCode::Null => {KeyCode::Null},
        crossterm::event::KeyCode::Esc => {KeyCode::Esc},
        crossterm::event::KeyCode::CapsLock => {KeyCode::CapsLock},
        crossterm::event::KeyCode::ScrollLock => {KeyCode::ScrollLock},
        crossterm::event::KeyCode::NumLock => {KeyCode::NumLock},
        crossterm::event::KeyCode::PrintScreen => {KeyCode::PrintScreen},
        crossterm::event::KeyCode::Pause => {KeyCode::Pause},
        crossterm::event::KeyCode::Menu => {KeyCode::Menu},
        crossterm::event::KeyCode::KeypadBegin => {KeyCode::KeypadBegin},
        _ => {KeyCode::Esc},
    };

    let action = match event.kind {
        crossterm::event::KeyEventKind::Press => {InputAction::Press},
        crossterm::event::KeyEventKind::Repeat => {InputAction::Hold},
        crossterm::event::KeyEventKind::Release => {InputAction::Release},
    };

    Input {
        key_code,
        action,
    }
}