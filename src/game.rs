extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;


use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;

// CONSTANTS
pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1000;

pub const PLAYER_WIDTH: f64 = 40_f64;
pub const PLAYER_HEIGHT: f64 = 70_f64;

pub const BALL_SIZE: f64 = 20_f64;
pub const BALL_SPEED_Y_DEFAULT: f64 = 1_f64;
pub const BALL_SPEED_X_DEFAULT: f64 = 7_f64;
const TOLERANCE: f64 = 4_f64;
// END OF CONSTANTS


pub struct Game {
    pub gl: GlGraphics,
    pub ball: Ball,
    pub player1: Player,
    pub player2: Player,
    pub player1_points: u32,
    pub player2_points: u32,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(graphics::color::BLACK, gl)
        });

        self.ball.draw(&mut self.gl, args);

        self.player1.draw(&mut self.gl, args);
        self.player2.draw(&mut self.gl, args);
    }

    pub fn update(&mut self) {
        if (!(self.player1.y <= 20_f64) && self.player1.direction == Direction::Up)
            || (!(self.player1.y >= SCREEN_HEIGHT as f64 - PLAYER_HEIGHT - 20_f64)
                && self.player1.direction == Direction::Down)
        {
            self.player1.y += match self.player1.direction {
                Direction::Up => -self.player1.speed,
                Direction::Down => self.player1.speed,
                _ => 0_f64,
            }
        }
        if (!(self.player2.y <= 20_f64) && self.player2.direction == Direction::Up)
            || (!(self.player2.y >= SCREEN_HEIGHT as f64 - PLAYER_HEIGHT - 20_f64)
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

    pub fn pressed(&mut self, btn: &Button) {
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

    pub fn released(&mut self, btn: &Button) {
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

    pub fn check_collision(&mut self) {
        if self.ball.y <= 0_f64 {
            self.ball.collision(Direction::Up)
        }
        else if self.ball.y >= SCREEN_HEIGHT as f64 - BALL_SIZE {
            self.ball.collision(Direction::Down)
        }
        else if self.ball.x >= self.player1.x + PLAYER_WIDTH - TOLERANCE
            && self.ball.x <= self.player1.x + PLAYER_WIDTH + TOLERANCE
            && self.ball.y <= self.player1.y + PLAYER_HEIGHT - BALL_SIZE/2_f64
            && self.ball.y >= self.player1.y - BALL_SIZE/2_f64 {
                self.ball.collision_player(Direction::Left, self.player1.direction)
        }
        else if self.ball.x >= self.player2.x - BALL_SIZE - TOLERANCE
            && self.ball.x <= self.player2.x - BALL_SIZE + TOLERANCE
            && self.ball.y <= self.player2.y + PLAYER_HEIGHT - BALL_SIZE/2_f64
            && self.ball.y >= self.player2.y - BALL_SIZE/2_f64 {
                self.ball.collision_player(Direction::Right, self.player2.direction)
        }

        if (self.ball.x + BALL_SIZE/2_f64 <= self.player1.x + PLAYER_WIDTH
            && self.ball.x + BALL_SIZE/2_f64 >= self.player1.x)
            && ((self.ball.y + BALL_SIZE >= self.player1.y - TOLERANCE
            && self.ball.y + BALL_SIZE <= self.player1.y + TOLERANCE)
            || ( self.ball.y >= self.player1.y + PLAYER_HEIGHT - TOLERANCE
            && self.ball.y <= self.player1.y + PLAYER_HEIGHT + TOLERANCE))
        {
                self.ball.speedy += match self.player1.direction {
                    Direction::Down => self.player1.speed,
                    Direction::Up => self.player1.speed,
                    _ => 0_f64,
                };
                if self.ball.y  >= self.player1.y + PLAYER_HEIGHT/2_f64 {
                    self.ball.end_collision(Direction::Down)
                }
                else {
                    self.ball.end_collision(Direction::Up)
                }
        }
        else if (self.ball.x + BALL_SIZE/2_f64 <= self.player2.x + PLAYER_WIDTH
            && self.ball.x + BALL_SIZE/2_f64 >= self.player2.x)
            && ((self.ball.y + BALL_SIZE >= self.player2.y - TOLERANCE
            && self.ball.y + BALL_SIZE <= self.player2.y + TOLERANCE)
            || ( self.ball.y >= self.player2.y + PLAYER_HEIGHT - TOLERANCE
            && self.ball.y <= self.player2.y + PLAYER_HEIGHT + TOLERANCE))
        {
                self.ball.speedy += match self.player2.direction {
                    Direction::Down => self.player2.speed,
                    Direction::Up => self.player2.speed,
                    _ => 0_f64,
                };
                if self.ball.y  >= self.player2.y + PLAYER_HEIGHT/2_f64 {
                    self.ball.end_collision(Direction::Down)
                }
                else {
                    self.ball.end_collision(Direction::Up)
                }
        }
        else if self.ball.x >= self.player1.x + PLAYER_WIDTH - BALL_SIZE
            && self.ball.x <= self.player1.x + PLAYER_WIDTH
            && self.ball.y >= self.player1.y + PLAYER_HEIGHT - BALL_SIZE
            && self.ball.y <= self.player1.y + PLAYER_HEIGHT
        {
            self.ball.corner_collision(Direction::Left, Direction::Up)
        }
        else if self.ball.x >= self.player2.x + PLAYER_WIDTH - BALL_SIZE
            && self.ball.x <= self.player2.x + PLAYER_WIDTH
            && self.ball.y >= self.player2.y - BALL_SIZE
            && self.ball.y <= self.player2.y
        {
            self.ball.corner_collision(Direction::Left, Direction::Down)
        }
    }

    pub fn check_oob(&mut self) {
        if self.ball.x + BALL_SIZE + 2_f64 < 0_f64 {
            self.player2_points += 1;
            self.ball = Ball::new();
        }
        else if self.ball.x > (SCREEN_WIDTH +2) as f64
        {
            self.player1_points += 1;
            self.ball = Ball::new();
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Still,
}

pub struct Ball {
    x: f64,
    y: f64,
    speedx: f64,
    speedy: f64,
    x_dir: Direction,
    y_dir: Direction,
}

impl Ball {
    pub fn new() -> Self {
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

    fn collision(&mut self, what: Direction) {
        match what {
            Direction::Up => self.y_dir = Direction::Down,
            Direction::Down => self.y_dir = Direction::Up,

            _ => (),
        }
    }

    fn collision_player(&mut self, what: Direction, what_direction: Direction) {
        if self.y_dir == what_direction {
            self.speedy *= 1.3;
        }
        else if self.y_dir != what_direction && what_direction != Direction::Still {
            self.speedy /= 1.25;
        }

        match what {
            Direction::Right => {
                self.x_dir = Direction::Left;
            },
            Direction::Left=> {
                self.x_dir = Direction::Right;
            }
            _ => (),
        }
    }

    fn corner_collision(&mut self, what_side: Direction, top_or_bottom: Direction) {
        self.x_dir = match what_side {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => Direction::Still,
        };
        self.y_dir = match top_or_bottom {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            _ => Direction::Still,
        };
        ()
    }

    fn end_collision(&mut self, end: Direction) {
        self.y_dir = end;
    }
}

pub struct Player {
    x: f64,
    y: f64,
    speed: f64,
    color: [f32; 4],
    direction: Direction,
}

impl Player {
    pub fn new(x: f64, y: f64, speed: f64, color: [f32; 4], direction: Direction) -> Self {
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