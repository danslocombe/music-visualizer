use common::GArg;
use graphics::opengl_graphics::GlGraphics;
use graphics::piston::input::RenderArgs;
use graphics::Background;
use graphics::common::*;
use std::collections::HashMap;

pub struct SolidColor {
    vars: HashMap<GArg, f64>
}

impl SolidColor {
    pub fn new() -> Self {
        let vars = make_map![GArg::R,0.0;GArg::G,0.0;GArg::B,0.0;GArg::Trans,1.0];

        SolidColor {
            vars: vars,
        }
    }
}

impl Background for SolidColor {
    fn render(&self, gl_graphics: &mut GlGraphics, args: &RenderArgs) {
        use graphics::graphics::clear;

        let fill_color = cons_color(&self.vars);

        gl_graphics.draw(args.viewport(), |_,gl| {
            clear(fill_color, gl);
        });
    }

    fn update(&mut self, args: &[(GArg, f64)]) {
        for (a,v) in args.iter().cloned() {
            self.vars.insert(a,v);
        }
    }
}
