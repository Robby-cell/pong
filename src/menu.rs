extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;


use graphics::color;
use opengl_graphics::GlGraphics;
use piston::input::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::TextureSettings;
use opengl_graphics::Filter;
use graphics::text;
use crate::graphics::Transformed;


pub struct Menu<'a> {
    pub gl: GlGraphics,
    pub score_p1: String,
    pub score_p2: String,

    instructions1: &'a str,
    instructions2: &'a str,
    instructions3: &'a str,
}

impl<'a> Menu<'a> {

    pub fn new(gl: GlGraphics, score_p1: String, score_p2: String) -> Self {
        Menu {
            gl,
            score_p1,
            score_p2,

            instructions1: "12345678901234567890",
            instructions2: "test 2",
            instructions3: "test 3",
        }
    }

    pub fn clear_screen(&mut self) {
        graphics::clear(color::BLACK, &mut self.gl)
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(800., 780.);
            let texture_settings = TextureSettings::new().filter(Filter::Nearest);

            let ref mut glyphs = GlyphCache::new("assets/text.ttf", (), texture_settings)
                .expect("Could not load font");


            graphics::text([0., 1., 0., 1.],
                32,
                self.instructions1,
                glyphs,
                transform,
                gl).unwrap();

            /*
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                self.instructions,
                glyphs,
                &c.draw_state,
                transform, gl
            ).unwrap();
            */
        })
    }
}