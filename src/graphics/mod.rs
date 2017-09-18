extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{Colored, GlGraphics, OpenGL, Textured};
use self::piston::event_loop::*;
use self::piston::input::*;
use self::piston::window::WindowSettings;

use common::GraphicsPacket;
use std::sync::mpsc::Receiver;
use std::time::{Duration, SystemTime};

mod geom_visuals;

use self::geom_visuals::*;

type Color = [f32; 4];

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];


fn color_from_val(x: f64) -> Color {
    let y = x as f32;
    // Set alpha to 1
    [y, y, y, 1.0]
}

// trait for visualising a single effect
trait Visualization {
    fn update(&mut self, args: &[f64], args_time: Duration);
    fn render(&self, fps: f64, gl_graphics: &mut GlGraphics, args: &RenderArgs);
}

pub struct ActiveEffects {
    effects: Vec<Box<Visualization>>,
    // background?
}

impl ActiveEffects {
    fn update_all(&mut self, mut update_buffer: Vec<GraphicsPacket>) {
        let (effect_args, packet_time) = match update_buffer.pop() {
            Some(p) => (p.effect_args, p.time),
            None => (vec![Vec::new(); self.effects.len()], Duration::new(0, 0)),
        };

        //let effect_args = latest_packet.effect_args;

        //let ref mut effects = self.effects;

        for (i, e) in self.effects.iter_mut().enumerate() {
            e.update(&effect_args[i], packet_time);
        }
    }

    fn render_all(
        &self,
        fps: f64,
        gl_graphics: &mut GlGraphics,
        args: &RenderArgs,
    ) {
        use graphics::graphics::clear;

        // For some reason this is bugging out I think?
        // I don't know, you get multiple rings and it looks really cool
        // (but you shouldn't)
        gl_graphics.draw(args.viewport(), |_, gl| { clear(BLACK, gl); });

        for e in &self.effects {
            e.render(fps, gl_graphics, args);
        }
    }
}

pub fn run(start_time: SystemTime, rx: &Receiver<GraphicsPacket>) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V3_3;

    let mut window: Window = WindowSettings::new("Simon", [800, 600])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let c = Colored::new(opengl.to_glsl());
    let t = Textured::new(opengl.to_glsl());

    let mut gl_graphics = GlGraphics::from_colored_textured(c, t);

    //let mut visuals = CircleVisuals::new(start_time);
    let mut visuals: Vec<Box<Visualization>> = Vec::new();
    let mut prev_time = SystemTime::now();

    //visuals- later on, this will init in the interpreter
    let c_white = CircleVisuals::new(start_time, WHITE, 1.0);
    visuals.push(Box::new(c_white));
    let c_red = DotsVisuals::new(RED, 8, 1.0);
    visuals.push(Box::new(c_red));

    let mut ae = ActiveEffects { effects: visuals };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => {

                // Calculate fps
                let dt = prev_time.elapsed().unwrap();
                prev_time = SystemTime::now();
                let fps = f64::from(1_000_000_000 / dt.subsec_nanos());

                ae.render_all(fps, &mut gl_graphics, &r);
            }
            Input::Update(_) => {

                // Get all the pending updates from the receiver and buffer into list
                let update_buffer = rx.try_iter()
                    .collect::<Vec<GraphicsPacket>>();

                ae.update_all(update_buffer);
            }
            Input::Press(i) => {
                // Ignore for now
            }
            _ => {}
        }
    }

}
