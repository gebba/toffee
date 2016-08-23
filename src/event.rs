
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use terminal::Terminal;

pub struct Events {
    pub event_pump: sdl2::EventPump,
}

impl Events {
    pub fn new(terminal: &Terminal) -> Self {
        let event_pump = terminal.sdl_context.event_pump().unwrap();
        Events { event_pump: event_pump }
    }
}
