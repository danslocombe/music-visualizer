extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::sync::mpsc::Receiver;

use common::GraphicsPacket;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{Colored, GlGraphics, OpenGL, Textured};
use self::piston::event_loop::*;
use self::piston::window::WindowSettings;
use self::piston::input::*;
use std::time::{Duration, SystemTime};

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
trait Visualization {
    fn update(&mut self, args: &Vec<f64>, args_time: Duration);
    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs);
}

pub struct CircleVisuals {
    start_time : SystemTime,
    last_trigger : Duration,
    last_trigger_value : f64,
    since_last : u32,
    on : bool,
    color: Color,
    multiplier: f64,
}

impl CircleVisuals {
    pub fn new(start_time: SystemTime, color: Color, mult: f64) -> Self {
        CircleVisuals {
            start_time : start_time,
            since_last : 0,
            last_trigger : Duration::new(0, 0),
            last_trigger_value : 0.0,
            on : false,
            color: color,
            multiplier: mult
        }
    }
}

impl Visualization for CircleVisuals {

    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl_graphics.draw(args.viewport(), |_, gl| {
            // Circle precision
            let prec : i32 = 32;
            let prec_d = prec as f64;

            // Circle radius
            let r_mult = self.last_trigger_value.abs() * self.multiplier.abs();

            let r = if self.on {
                r_mult
            }
            else {
                r_mult / (self.since_last as f64)
            } * 2.0;

            const TWO_PI : f64 = 6.282;

            // Draw a circle of radius r
            for i in 0..prec {
                let i_d = i as f64;
                let i1_d = (i + 1) as f64;
                let p1 = Point{
                    x : r * (TWO_PI * i_d / prec_d).cos(),
                    y : r * (TWO_PI * i_d / prec_d).sin(),
                };
                let p2 = Point{
                    x : r * (TWO_PI * i1_d / prec_d).cos(),
                    y : r * (TWO_PI * i1_d / prec_d).sin(),
                };
                line_points(gl, args, self.color, 1.0, &p1, &p2);
            }
        });
    }

    fn update(&mut self, args: &Vec<f64>, args_time: Duration) {
        self.since_last = self.since_last + 1;

        if args.len() > 0 {
            self.since_last = 0;
            self.last_trigger = args_time;
            self.last_trigger_value = args[0];
        }

        //self.on = false;

        // Only update if song is playing
        let _ = self.start_time.elapsed().map(|current_time| {

            // 50 milliseconds
            let epilepsy_preventation_duration = Duration::new(0, 50_000_000);

            let since_trigger = diff_durs(&current_time, &self.last_trigger);

            self.on = since_trigger < epilepsy_preventation_duration;
        });
    }
}

pub struct ActiveEffects {
    effects: Vec<Box<Visualization>>,
    // background?
}

impl ActiveEffects {
    fn update_all(&mut self, mut update_buffer: Vec<GraphicsPacket>) {
        let (effect_args, packet_time) = match update_buffer.pop() {
            Some(p) => (p.effect_args, p.time),
            None => (vec![Vec::new();self.effects.len()], Duration::new(0,0))
        };
    
        //let effect_args = latest_packet.effect_args;

        //let ref mut effects = self.effects;
    
        for (i, e) in self.effects.iter_mut().enumerate() {
            e.update(&effect_args[i], packet_time);
        }
    }

    fn render_all(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::graphics::clear;

        // For some reason this is bugging out I think?
        // I don't know, you get multiple rings and it looks really cool
        // (but you shouldn't)
        gl_graphics.draw(args.viewport(), |_, gl| {
            clear(BLACK, gl);
        });
        /*let _ = self.effects.iter()
                    .map(|v| v.render(fps, gl_graphics, args));*/

        for e in self.effects.iter() {
            e.render(fps, gl_graphics, args);
        }
    }
}

// The difference between two durations
fn diff_durs(x : &Duration, y : &Duration) -> Duration {
    let (greater, lesser) = if x > y {
        (x, y)
    }
    else {
        (y, x)
    };
    *greater - *lesser
}

struct Point {x : f64, y : f64}

// Taken from another project
fn line_points (gl : &mut GlGraphics,
                args: &RenderArgs,
                color : Color,
                width: f64,
                p1: &Point,
                p2: &Point) {
    use graphics::*;

    let camera_scale = (args.width / 2) as f64;

    let (x_mid, y_mid) = ((args.width / 2) as f64, (args.height / 2) as f64);

    let Point{x : x1, y : y1} = *p1;
    let Point{x : x2, y : y2} = *p2;

    let x_start = x_mid + x1 * camera_scale;
    let y_start = y_mid + y1 * camera_scale;

    let x_end = x_mid + x2 * camera_scale;
    let y_end = y_mid + y2 * camera_scale;

    gl.draw(args.viewport(), |c, gl| {
        let transform = c.transform;
        let l = [x_start, y_start, x_end, y_end];
        graphics::line(color, width, l, transform, gl);
    });
}

pub fn run(start_time : SystemTime, rx : Receiver<GraphicsPacket>) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V3_3;

    let mut window : Window = WindowSettings::new("Simon", [800, 600])
        .opengl(opengl)
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
    let c_red = CircleVisuals::new(start_time, RED, 0.1);
    visuals.push(Box::new(c_red));

    let mut ae = ActiveEffects { effects: visuals };


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
