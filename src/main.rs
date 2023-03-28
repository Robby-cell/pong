extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod game;
mod menu;

use glutin_window::GlutinWindow;
//use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*, input::*, window::WindowSettings};

use game::*;
use menu::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opengl = OpenGL::V4_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong!", [SCREEN_WIDTH, SCREEN_WIDTH])
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .ok()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        paused: false,
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

    let mut menu = Menu::new(GlGraphics::new(OpenGL::V4_4), 0, 0);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            if !game.paused {
                game.check_oob();
                game.check_collision();
                game.render(&r);
                game.update();
            } else {
                //menu.clear_screen();
                game.render(&r);
                *menu.score_p1 = game.player1_points;
                *menu.score_p2 = game.player2_points;
                
                menu.render(&r, (752., 600.), menu.instruction1)?;
                menu.render(&r, (1574., 120.), menu.instruction2)?;
            }
        };

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                if !game.paused || &k.button == &Button::Keyboard(Key::Space) {
                    game.pressed(&k.button);
                }
                else {
                    menu.pressed(&k.button);
                }
            }

            if k.state == ButtonState::Release {
                if !game.paused {
                    game.released(&k.button);
                }
                else {
                    // something
                }
            }
        };
    }

    Ok(())

}
