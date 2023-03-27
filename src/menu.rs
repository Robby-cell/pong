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
    pub score_p1: &'a str,
    pub score_p2: &'a str,

    pub instructions: &'a str,

}

impl<'a> Menu<'a> {

    pub fn clear_screen(&mut self) {
        graphics::clear(color::BLACK, &mut self.gl)
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(800., 780.);
            let texture_settings = TextureSettings::new().filter(Filter::Nearest);

            let ref mut glyphs = GlyphCache::new("assets/text.ttf", (), texture_settings)
                .expect("Could not load font");


            graphics::text(color::WHITE, 32, self.instructions, glyphs, transform, gl).unwrap();
            /*
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "press space to quit, g to continue",
                glyphs,
                &c.draw_state,
                transform, gl
            ).unwrap();*/
        })
    }
}