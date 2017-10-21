extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::sync::mpsc::Receiver;

use common::{GArg, GraphicsPacket, GraphicsUpdate};
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{Colored, GlGraphics, OpenGL, Textured};
use self::piston::event_loop::*;
use self::piston::window::WindowSettings;
use self::piston::input::*;
use std::time::{Duration, SystemTime};

#[macro_use]
mod common;
pub mod geom_visuals;
pub mod backgrounds;


// trait for visualising a single effect
pub trait Visualization: Send {
    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration);
    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs);
}

// trait for backgrounds
pub trait Background: Send {
    fn update(&mut self, args: &[(GArg, f64)]);
    fn render(&self, gl_graphics : &mut GlGraphics, args: &RenderArgs);
}

pub struct ActiveEffects {
    pub bg: Box<Background>,
    pub effects: Vec<Box<Visualization>>,
}

impl ActiveEffects {
    fn update_all(&mut self, update: GraphicsUpdate) {
        let (bg_args, effect_args, packet_time) = (update.bg_args, update.effect_args, update.time);

        self.bg.update(&bg_args);
    
        for (i, e) in self.effects.iter_mut().enumerate() {
            e.update(&effect_args[i], packet_time);
        }
    }

    fn render_all(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs, window: &mut Window) {
        use graphics::graphics::clear;

        // draw background
        self.bg.render(gl_graphics, args);

        // draw effects in order
        for e in self.effects.iter() {
            e.render(fps, gl_graphics, args);
        }

        /*let texture = Texture::from_image(
            &mut window.factory,
            & [image]
            &TextureSettings::new()
        ).unwrap();;

        window.draw_2d(&e, |c, gl| {
            clear(BLACK, gl);
            image(&texture, c.transform, gl);
        });*/
    }
}

pub fn run(start_time : SystemTime, rx : Receiver<GraphicsPacket>, effects: ActiveEffects) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V3_3;

    let mut window : Window = WindowSettings::new("Audisuals", [800, 600])
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
                let fps = 1000_000_000.0 / (dt.subsec_nanos() as f64); // TODO: is this necessary?

                ae.render_all(fps, &mut gl_graphics, &r, &mut window);
            }
            Input::Update(_) => {

                // Get all the pending updates from the receiver and buffer into list
                let mut update_buffer = rx.try_iter().collect::<Vec<GraphicsPacket>>();

                match update_buffer.pop() {
                    Some(GraphicsPacket::Update(update)) => ae.update_all(update),
                    Some(GraphicsPacket::Refresh(effects)) => ae = effects,
                    None => {
                        let len = ae.effects.len();
                        ae.update_all(GraphicsUpdate::new_empty(len))
                    },
                }

                //ae.update_all(update_buffer);
            }
            Input::Press(i) => {
                // Ignore for now
            }
            _ => {}
        }
    }

}
