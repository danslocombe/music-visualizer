extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::sync::mpsc::Receiver;

use common::{GArg, GraphicsPacket};
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{Colored, GlGraphics, OpenGL, Textured};
use self::piston::event_loop::*;
use self::piston::window::WindowSettings;
use self::piston::input::*;
use std::time::{Duration, SystemTime};

pub mod geom_visuals;

use self::geom_visuals::*;

type Color = [f32; 4];

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];


fn color_from_val(x : f64) -> Color {
    let y = x as f32;
    // Set alpha to 1
    [y, y, y, 1.0]
}

// trait for visualising a single effect
pub trait Visualization: Send {
    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration);
    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs);
}

pub struct ActiveEffects {
    pub effects: Vec<Box<Visualization>>,
    // TODO: background
}

impl ActiveEffects {
    fn update_all(&mut self, mut update_buffer: Vec<GraphicsPacket>) {
        let (effect_args, packet_time) = match update_buffer.pop() {
            Some(p) => (p.effect_args, p.time),
            None => (vec![Vec::new();self.effects.len()], Duration::new(0,0))
        };
    
        for (i, e) in self.effects.iter_mut().enumerate() {
            e.update(&effect_args[i], packet_time);
        }
    }

    fn render_all(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::graphics::clear;

        // draw background
        gl_graphics.draw(args.viewport(), |_, gl| {
            clear(BLACK, gl);
        });

        // draw effects in order
        for e in self.effects.iter() {
            e.render(fps, gl_graphics, args);
        }
    }
}

pub fn run(start_time : SystemTime, rx : Receiver<GraphicsPacket>, effects: ActiveEffects) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V3_3;

    let mut window : Window = WindowSettings::new("Simon", [800, 600])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let c = Colored::new(opengl.to_glsl());
    let t = Textured::new(opengl.to_glsl());

    let mut gl_graphics = GlGraphics::from_colored_textured(c, t);

    let mut prev_time = SystemTime::now();

    let mut ae = effects;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => {

                // Calculate fps
                let dt = prev_time.elapsed().unwrap();
                prev_time = SystemTime::now();
                let fps = 1000_000_000.0 / (dt.subsec_nanos() as f64);

                ae.render_all(fps, &mut gl_graphics, &r);
            }
            Input::Update(_) => {

                // Get all the pending updates from the receiver and buffer into list
                let update_buffer = rx.try_iter().collect::<Vec<GraphicsPacket>>();

                ae.update_all(update_buffer);
            }
            Input::Press(i) => {
                // Ignore for now
            }
            _ => {}
        }
    }

}
