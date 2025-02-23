use std::io::stdout;

use crossterm::{
    cursor::{
        Hide, 
        MoveTo, 
        Show
    }, 
    execute, 
    style::Print, 
    terminal::{
        self, 
        disable_raw_mode, 
        enable_raw_mode, 
        Clear, 
        ClearType, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }, 
    ExecutableCommand,
};

use crate::backends::traits::{BackendTrait, DrawerTrair, ListenerTrait};
use crate::{
    layout::{
        Container, 
        StyleLink
    }, 
    style::{
        Style,
        Align, 
        ContentWrap, 
        Orientation, 
        Size, 
    }
};

use super::{drawer::Drawer, listener::{self, Listener}};


pub struct Backend<D: DrawerTrair, L: ListenerTrait> {
    pub drawer: D,
    pub listener: L,
}

impl<D: DrawerTrair, L: ListenerTrait> Backend<D, L> {
    pub fn new(drawer: D, listener: L) -> Self {
        Backend {
            drawer,
            listener,
        }
    }
}

impl<D: DrawerTrair, L: ListenerTrait> BackendTrait for Backend<D, L> {
    fn init(&mut self) {
        let _ = enable_raw_mode();
        self.listener.watch_input();
        let _ = execute!(stdout(), EnterAlternateScreen);
    }

    fn close(& self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let mut stdout = stdout();
        stdout.execute(Show);
    }

    fn render(&mut self, main_container: Container, style_links: Vec<StyleLink>) {
        self.drawer.render(main_container, style_links);
    }

    fn display(&self) {
        self.drawer.display();
    }
}
