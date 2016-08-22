extern crate sdl2;
extern crate toffee;

use std::time;
use toffee::font::FontDefinition;
use toffee::terminal::Terminal;
use toffee::renderer::Renderer;
use toffee::colors;

pub fn main() {
    let font = FontDefinition {
        image_path: "fonts/taffer.png",
        width: 20,
        height: 20,
        padding: 0,
        transparent: false,
    };
    let mut term = Terminal::new(50, 20);
    let mut term_renderer = Renderer::new(font, 50, 20);

    term.print_center(5, "It seems to be working!", colors::WHITE, colors::BLUE);

    let mut last_update = time::SystemTime::now();
    let mut frames = 0;
    let mut fps = 0;

    let mut running_time = 10;

    'mainloop: loop {
        if running_time <= 0 {
            break 'mainloop;
        }
        term.print(0,
                   0,
                   format!("FPS: {}", fps).as_str(),
                   colors::YELLOW,
                   colors::BLACK);

        term.print_center(6,
                          format!("   Closing in {} seconds.   ", running_time).as_str(),
                          colors::RED,
                          colors::BLACK);

        term_renderer.draw(&term);
        frames += 1;

        let now = time::SystemTime::now();
        let seconds_since_last =
            time::SystemTime::now().duration_since(last_update).unwrap().as_secs();

        running_time -= seconds_since_last;
        if seconds_since_last >= 1 {
            fps = frames;
            frames = 0;
            last_update = now;
        }
    }
}
