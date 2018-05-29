extern crate sdl2;
extern crate toffee;

use std::time;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use toffee::font::FontDefinition;
use toffee::terminal::Terminal;
use toffee::renderer::Renderer;
use toffee::event::Events;
use toffee::colors;

pub fn main() {
    let font = FontDefinition {
        image_path: "fonts/grim_10x12.png",
        width: 10,
        height: 12,
        padding: 0,
        transparent: false,
    };
    let mut term = Terminal::new(50, 20);
    let mut term_renderer = Renderer::new(&term, font);
    let mut event_handler = Events::new(&term);

    term.print_center(5, "It seems to be working!", colors::WHITE, colors::BLUE);

    let mut last_update = time::SystemTime::now();
    let mut frames = 0;
    let mut fps = 0;

    'mainloop: loop {

        for event in event_handler.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'mainloop,
                _ => {}
            }
        }

        term.print(0,
                   0,
                   format!("FPS: {}", fps).as_str(),
                   colors::YELLOW,
                   colors::BLACK);

        term_renderer.draw(&mut term);
        frames += 1;

        let now = time::SystemTime::now();
        let seconds_since_last =
            time::SystemTime::now().duration_since(last_update).unwrap().as_secs();

        if seconds_since_last >= 1 {
            fps = frames;
            frames = 0;
            last_update = now;
        }
    }
}
