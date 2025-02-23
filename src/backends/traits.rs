use crate::layout::{Container, StyleLink};

pub trait BackendTrait {
    fn init(&mut self);
    fn close(& self);
    fn render(&mut self, main_container: Container, style_links: Vec<StyleLink>);
    fn display(&self);
}

pub trait DrawerTrair {
    fn render(&mut self, main_container: Container, style_links: Vec<StyleLink>);
    fn display(&self);
}

pub trait ListenerTrait {
    fn watch_input(& self);
}