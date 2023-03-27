extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use crate::graphics::Transformed;
use graphics::color;
use graphics::glyph_cache::rusttype::GlyphCache;
use graphics::text;
use opengl_graphics::Filter;
use opengl_graphics::GlGraphics;
use opengl_graphics::TextureSettings;
use piston::input::*;

pub struct Menu<'a> {
    pub gl: GlGraphics,
    pub score_p1: Box<u32>,
    pub score_p2: Box<u32>,

    instructions1: &'a str,
    instructions2: &'a str,
    instructions3: &'a str,
}

impl<'a> Menu<'a> {
    pub fn new(gl: GlGraphics, score_p1: u32, score_p2: u32) -> Self {
        Menu {
            gl,
            score_p1: Box::new(score_p1),
            score_p2: Box::new(score_p2),

            instructions1: "12345678901234567890",
            instructions2: "test 2",
            instructions3: "test 3",
        }
    }

    pub fn update_score(&mut self, which_player: u8) {
        match which_player {
            1 => *self.score_p1 += 1,
            2 => *self.score_p2 += 1,
            _ => (),
        }
    }

    pub fn clear_screen(&mut self) {
        graphics::clear(color::BLACK, &mut self.gl)
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(800., 780.);
            let texture_settings = TextureSettings::new().filter(Filter::Nearest);

            let ref mut glyphs = GlyphCache::new("assets/text.ttf", (), texture_settings)?;
            //.expect("Could not load font");

            graphics::text(
                [0., 1., 0., 1.],
                32,
                self.instructions1,
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
}
