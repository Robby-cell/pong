extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::game::SCREEN_WIDTH;
use crate::graphics::Transformed;
use graphics::color;
use graphics::glyph_cache::rusttype::GlyphCache;
//use graphics::text;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::TextureSettings;
use piston::input::*;

pub const TEXT_SIZE: f64 = 32_f64;

pub struct Menu<'a> {
    pub gl: GlGraphics,

    pub instruction1: &'a str,
    pub instruction2: &'a str,
    pub instruction3: &'a str,
}

impl<'a> Menu<'a> {
    pub fn new(gl: GlGraphics) -> Self {
        Menu {
            gl,

            instruction1: "Resume [space] ",
            instruction2: "Quit [Q] ",
            instruction3: "Scoreboard [S] ",
        }
    }

    #[allow(unused)]
    pub fn clear_screen(&mut self) {
        //
        // THIS FUNCTION MOST LIKELY WILL NOT BE USED SINCE WE WANT TO DISPLAY THE GAME IN THE BACKGROUND
        // SO CLEARING THE SCREEN SIMPLY WOULD NOT MAKE SENSE
        //
        graphics::clear(color::BLACK, &mut self.gl)
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        position: (f64, f64),
        what: String,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.gl.draw(args.viewport(), |c, gl| {
            let transform: [[f64; 3]; 2] = c.transform.trans(position.0, position.1);
            let texture_settings: TextureSettings = TextureSettings::new().filter(Filter::Nearest);

            let ref mut glyphs = GlyphCache::new("assets/text.ttf", (), texture_settings)?;
            //.expect("Could not load font"); // We wont use this, since we have '?', which will exit function if it wants to give us an error

            graphics::text(
                [0., 0.3, 0.6, 1.],
                TEXT_SIZE as u32,
                &what,
                glyphs,
                transform,
                gl,
            )?;

            /*
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                self.instructions,
                glyphs,
                &c.draw_state,
                transform, gl
            ).unwrap();
            */
            Ok(())
        })
    }

    pub fn pressed(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Q) => std::process::exit(0),

            _ => (),
        }
    }

    #[allow(warnings)]
    pub fn get_centre(&self, word: &'a str, letter_size: f64) -> f64 {
        SCREEN_WIDTH as f64 / 2_f64 - ((word.len() + 1) as f64 * letter_size) / 2_f64
    }
}
