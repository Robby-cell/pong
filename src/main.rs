extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod game;
mod menu;
//mod leaderboard;

use glutin_window::GlutinWindow;
//use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*, input::*, window::WindowSettings};

use crate::game::*;
use crate::menu::*;

enum GameState {
    Running,
    Paused,
    Leaderboard,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opengl: OpenGL = OpenGL::V4_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong!", [SCREEN_WIDTH, SCREEN_WIDTH])
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .ok()
        .unwrap();

    let mut game: Game = Game {
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

    //let mut lb = leaderboard::Leaderboard::new()?; // we are not currently using this feature

    let mut events: Events = Events::new(EventSettings::new().ups(60));

    let mut menu: Menu = Menu::new(GlGraphics::new(OpenGL::V4_4));

    let mut state: GameState = GameState::Running;

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            match state {
                GameState::Running => {
                    game.check_oob();
                    game.check_collision();
                    game.render(&r);
                    game.update();
                }
                GameState::Paused => {
                    game.render(&r);

                    menu.render(
                        &r,
                        (menu.get_centre(menu.instruction1, TEXT_SIZE), 600.),
                        format!("{}", menu.instruction1),
                    )?;
                    menu.render(
                        &r,
                        (menu.get_centre(menu.instruction2, TEXT_SIZE), 400.),
                        format!("{}", menu.instruction2),
                    )?;

                    menu.render(
                        &r,
                        (TEXT_SIZE * 3_f64, 150.),
                        format!("{} ", game.player1.points),
                    )?;

                    let p2pnts = format!("{} ", game.player2.points);
                    menu.render(
                        &r,
                        (
                            SCREEN_WIDTH as f64
                                - TEXT_SIZE * 3_f64
                                - (p2pnts.len() as f64 * TEXT_SIZE),
                            150.,
                        ),
                        p2pnts,
                    )?;
                }
                GameState::Leaderboard => (),
            }
        };

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match state {
                    GameState::Running => {
                        if &k.button == &Button::Keyboard(Key::Space) {
                            state = GameState::Paused;
                        } else {
                            game.pressed(&k.button);
                        }
                    }
                    GameState::Paused => {
                        if &k.button == &Button::Keyboard(Key::Space) {
                            state = GameState::Running;
                        } else {
                            menu.pressed(&k.button);
                        }
                    }
                    GameState::Leaderboard => (),
                }
            }

            if k.state == ButtonState::Release {
                match state {
                    GameState::Running => game.released(&k.button),
                    GameState::Paused => (),
                    GameState::Leaderboard => (),
                }
            }
        };
    }

    Ok(())
}
