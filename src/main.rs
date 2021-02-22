extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(GREEN, gl);
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Creates a Glutin Window
    let mut window: Window = WindowSettings::new("Snake", [200, 200])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    // Event loop to run the game
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
