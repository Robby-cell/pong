extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*,
    input::*,
    window::WindowSettings};
use rand::Rng;

// CONSTANTS
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1000;

const PLAYER_WIDTH: f64 = 40_f64;
const PLAYER_HEIGHT: f64 = 70_f64;

const BALL_SIZE: f64 = 20_f64;
const BALL_SPEED_Y_DEFAULT: f64 = 1_f64;
const BALL_SPEED_X_DEFAULT: f64 = 7_f64;
// END OF CONSTANTS

struct Game {
    gl: GlGraphics,
    ball: Ball,
    player1: Player,
    player2: Player,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(graphics::color::BLACK, gl)
        });

        self.ball.draw(&mut self.gl, args);

        self.player1.draw(&mut self.gl, args);
        self.player2.draw(&mut self.gl, args);
    }

    fn update(&mut self) {
        if (!(self.player1.y <= 40_f64) && self.player1.direction == Direction::Up)
            || (!(self.player1.y >= SCREEN_HEIGHT as f64 - PLAYER_HEIGHT - 40_f64)
                && self.player1.direction == Direction::Down)
        {
            self.player1.y += match self.player1.direction {
                Direction::Up => -self.player1.speed,
                Direction::Down => self.player1.speed,
                _ => 0_f64,
            }
        }
        if (!(self.player2.y <= 40_f64) && self.player2.direction == Direction::Up)
            || (!(self.player2.y >= SCREEN_HEIGHT as f64 - PLAYER_HEIGHT - 40_f64)
                && self.player2.direction == Direction::Down)
        {
            self.player2.y += match self.player2.direction {
                Direction::Up => -self.player2.speed,
                Direction::Down => self.player2.speed,
                _ => 0_f64,
            }
        }

        self.ball.x += match self.ball.x_dir {
            Direction::Right => self.ball.speedx,
            Direction::Left => -self.ball.speedx,
            _ => 0_f64,
        };

        self.ball.y += match self.ball.y_dir {
            Direction::Up => -self.ball.speedy,
            Direction::Down => self.ball.speedy,
            _ => 0_f64,
        };
    }

    fn pressed(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::W) => {
                self.player1.direction = Direction::Up
            },
            &Button::Keyboard(Key::S) => {
                self.player1.direction = Direction::Down
            },

            &Button::Keyboard(Key::I) => {
                self.player2.direction = Direction::Up
            },
            &Button::Keyboard(Key::K) => {
                self.player2.direction = Direction::Down
            },

            _ => (),
        }
    }

    fn released(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::W) => {
                self.player1.direction = Direction::Still
            },
            &Button::Keyboard(Key::S) => {
                self.player1.direction = Direction::Still
            },

            &Button::Keyboard(Key::I) => {
                self.player2.direction = Direction::Still
            },
            &Button::Keyboard(Key::K) => {
                self.player2.direction = Direction::Still
            },

            _ => (),
        }
    }

    fn check_collision(&mut self) {
        if self.ball.y <= 0_f64 {
            self.ball.collision(Direction::Up, Direction::Still)
        }
        else if self.ball.y >= SCREEN_HEIGHT as f64 - BALL_SIZE {
            self.ball.collision(Direction::Down, Direction::Still)
        }
        else if self.ball.x >= self.player1.x + PLAYER_WIDTH - 5_f64
            && self.ball.x <= self.player1.x + PLAYER_WIDTH + 5_f64
            && self.ball.y <= self.player1.y + PLAYER_HEIGHT - BALL_SIZE/2_f64
            && self.ball.y >= self.player1.y - BALL_SIZE/2_f64 {
                self.ball.collision(Direction::Left, self.player1.direction)
        }
        else if self.ball.x >= self.player2.x - BALL_SIZE - 5_f64
            && self.ball.x <= self.player2.x - BALL_SIZE + 5_f64
            && self.ball.y <= self.player2.y + PLAYER_HEIGHT - BALL_SIZE/2_f64
            && self.ball.y >= self.player2.y - BALL_SIZE/2_f64 {
                self.ball.collision(Direction::Right, self.player2.direction)
        }

        if (self.ball.x + BALL_SIZE/2_f64 <= self.player1.x + PLAYER_WIDTH
            && self.ball.x + BALL_SIZE/2_f64 >= self.player1.x)
            && ((self.ball.y + BALL_SIZE >= self.player1.y - 5_f64
            && self.ball.y + BALL_SIZE <= self.player1.y + 5_f64)
            || ( self.ball.y >= self.player1.y + PLAYER_HEIGHT - 5_f64
            && self.ball.y <= self.player1.y + PLAYER_HEIGHT + 5_f64))
        {
                self.ball.speedy += match self.player1.direction {
                    Direction::Down => self.player1.speed,
                    Direction::Up => self.player1.speed,
                    _ => 0_f64,
                };
        }
        else if (self.ball.x + BALL_SIZE/2_f64 <= self.player2.x + PLAYER_WIDTH
            && self.ball.x + BALL_SIZE/2_f64 >= self.player2.x)
            && ((self.ball.y + BALL_SIZE >= self.player2.y - 5_f64
            && self.ball.y + BALL_SIZE <= self.player2.y + 5_f64)
            || ( self.ball.y >= self.player2.y + PLAYER_HEIGHT - 5_f64
            && self.ball.y <= self.player2.y + PLAYER_HEIGHT + 5_f64))
        {
                self.ball.speedy += match self.player2.direction {
                    Direction::Down => self.player2.speed,
                    Direction::Up => self.player2.speed,
                    _ => 0_f64,
                };
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Still,
}

struct Ball {
    x: f64,
    y: f64,
    speedx: f64,
    speedy: f64,
    x_dir: Direction,
    y_dir: Direction,
}

impl Ball {
    fn new() -> Self {
        let dirx = rand::thread_rng().gen_bool(0.5);
        let diry = rand::thread_rng().gen_bool(0.5);

        Ball {
            x: (SCREEN_WIDTH / 2) as f64,
            y: (SCREEN_HEIGHT / 2) as f64,
            speedx: BALL_SPEED_X_DEFAULT,
            speedy: BALL_SPEED_Y_DEFAULT,

            x_dir: match dirx {
                true => Direction::Right,
                false => Direction::Left,
            },
            y_dir: match diry {
                true => Direction::Up,
                false => Direction::Down,
            },
        }
    }

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let ball_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = graphics::rectangle::square(self.x, self.y, BALL_SIZE);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::ellipse(ball_color, square, transform, gl)
        })
    }

    fn collision(&mut self, what: Direction, what_direction: Direction) {
        match what {
            Direction::Up => self.y_dir = Direction::Down,
            Direction::Right => {
                self.x_dir = Direction::Left;

                if what_direction == Direction::Still {
                    ()
                }
                else if self.y_dir == what_direction {
                    self.speedy *= 1.3;
                }
                else {
                    self.speedy /= 1.25;
                }
            },
            Direction::Left=> {
                self.x_dir = Direction::Right;

                if what_direction == Direction::Still {
                    ()
                }
                else if self.y_dir == what_direction {
                    self.speedy *= 1.3;
                }
                else {
                    self.speedy /= 1.25;
                }
            }
            Direction::Down => self.y_dir = Direction::Up,
            _ => (),
        }
    }
}

struct Player {
    x: f64,
    y: f64,
    speed: f64,
    color: [f32; 4],
    direction: Direction,
}

impl Player {
    fn new(x: f64, y: f64, speed: f64, color: [f32; 4], direction: Direction) -> Self {
        Player { x, y, speed, color, direction }
    }

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let rectangle = graphics::rectangle::rectangle_by_corners(
            self.x,
            self.y,
            self.x + PLAYER_WIDTH,
            self.y + PLAYER_HEIGHT,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(self.color, rectangle, transform, gl)
        })
    }
}

fn main() {
    let opengl = OpenGL::V4_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong!", [SCREEN_WIDTH, SCREEN_WIDTH])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .ok()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        ball: Ball::new(),
        player1: Player::new(
            50_f64,
            (SCREEN_HEIGHT as f64 - PLAYER_HEIGHT) / 2_f64,
            7_f64,
            [0.0, 1.0, 0.0, 1.0],
            Direction::Still,
        ),
        player2: Player::new(
            SCREEN_WIDTH as f64 - (50_f64 + PLAYER_WIDTH),
            (SCREEN_HEIGHT as f64 - PLAYER_HEIGHT) / 2_f64,
            7_f64,
            [1.0, 0.0, 0.0, 1.0],
            Direction::Still,
        ),
    };

    let mut events = Events::new(EventSettings::new().ups(60));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.check_collision();
            game.render(&r);
            game.update();
        };

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }

            if k.state == ButtonState::Release {
                game.released(&k.button);
            }
        };
    }
}
