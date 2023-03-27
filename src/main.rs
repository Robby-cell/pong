extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod game;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*,
    input::*,
    window::WindowSettings};

use game::*;


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
        player1_points: 0,
        player2_points: 0,
    };

    let mut events = Events::new(EventSettings::new().ups(60));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.check_oob();
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
