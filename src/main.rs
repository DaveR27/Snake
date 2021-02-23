extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

#[derive(Clone, PartialEq)]
enum Direction {
    RIGHT, 
    LEFT, 
    UP, 
    DOWN,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    apple: Apple,
    score: i32,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            clear(BLACK, gl);
        });

        // Renders game components
        self.snake.render(&mut self.gl, arg);
        self.apple.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}

// Snake which is the player controlled input.
struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {

    // Renders the snake to the game board.
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(GREEN, square, transform, gl)
        });   
    }


    // Controls the direction of the snake
    fn update(&mut self) {
        match self.dir {
            Direction::LEFT => self.pos_x -= 1,
            Direction::RIGHT => self.pos_x += 1,
            Direction::UP => self.pos_y -= 1,
            Direction::DOWN => self.pos_y += 1,
        }
    }
}

// Apple which is used to score points.
struct Apple {
    points: i32,
    pos_x: i32,
    pos_y: i32,
}

impl Apple {

    fn render (&self, gl: &mut GlGraphics, args: &RenderArgs) {
        
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl)
        });   
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Creates a Glutin Window
    let mut window: Window = WindowSettings::new("Snake", [500, 500])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {pos_x: 1, pos_y: 1, dir: Direction::RIGHT},
        apple: Apple {pos_x: 5, pos_y: 5, points: 10},
        score: 0,
    };

    // Event loop to run the game
    let mut events = Events::new(EventSettings::new().ups(8));
    while let Some(e) = events.next(&mut window) {
        
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        } 
    }
}
