extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::sync::mpsc::Receiver;

use common::VisualizerUpdate;
use common::UpdateType::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{Colored, GlGraphics, OpenGL, Textured};
use self::piston::event_loop::*;
use self::piston::window::WindowSettings;
use self::piston::input::*;
use std::time::{Duration, SystemTime};

type Color = [f32; 4];

const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];


fn color_from_val(x : f64) -> Color {
    let y = x as f32;
    // Set alpha to 1
    [y, y, y, 1.0]
}

trait Visualization {
    fn update(&mut self, args: &UpdateArgs, update_buffer : Vec<VisualizerUpdate>);
    fn render(&mut self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs);
}

struct CircleVisuals {
    start_time : SystemTime,
    last_trigger : Duration,
    last_trigger_value : f64,
    level : f64,
    level_prev : f64,
    level_angle : f64,
    level_angle_prev : f64,
    level_beams : usize,
    since_last : u32,
    on : bool,
}

impl CircleVisuals {

    fn new(start_time : SystemTime) -> Self {
        CircleVisuals {
            start_time : start_time,
            since_last : 0,
            level : 0.0,
            level_prev : 0.0,
            level_angle : 0.0,
            level_angle_prev : 0.0,
            level_beams : 0,
            last_trigger : Duration::new(0, 0),
            last_trigger_value : 0.0,
            on : false,
        }
    }
}

impl Visualization for CircleVisuals {

    fn render(&mut self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        use graphics::graphics::clear;

        gl_graphics.draw(args.viewport(), |_, gl| {


            let threshhold = self.level > 0.6;
            let bg = if self.on {
                if threshhold {
                    color_from_val(1.0 - self.last_trigger_value)
                }
                else {
                    color_from_val(self.last_trigger_value)
                }
            }
            else {
                if threshhold {
                    WHITE
                }
                else {
                    BLACK
                }
            };

            // For some reason this is bugging out I think?
            // I don't know, you get multiple rings and it looks really cool
            // (but you shouldn't)
            clear(bg, gl);

            let color = if threshhold {
                BLACK
            }
            else {
                WHITE
            };

            // Circle precision
            let prec : i32 = 32;
            let prec_d = prec as f64;

            // Circle radius
            let r = if self.on {
                self.last_trigger_value.abs()
            }
            else {
                self.last_trigger_value.abs() / (self.since_last as f64)
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
                line_points(gl, args, color, 1.0, &p1, &p2);
            }

            // Draw beams
            for i in 0..self.level_beams {
                //let origin = Point{x : 0.0, y : 0.0};
                //
                let r0 = if threshhold {
                    self.level_prev * 0.8
                }
                else {
                    self.level_prev * 1.6
                };
                let theta0 = self.level_angle_prev + (i as f64) * TWO_PI  / (self.level_beams as f64);
                let p0 = Point {
                    x : r0 * (theta0).cos(),
                    y : r0 * (theta0).sin(),

                };

                let r = if threshhold {
                    self.level * 0.8
                }
                else {
                    self.level * 1.6
                };

                let theta = self.level_angle + (i as f64) * TWO_PI  / (self.level_beams as f64);
                let p = Point{
                    x : r * (theta).cos(),
                    y : r * (theta).sin(),
                };
                line_points(gl, args, color, 1.0, &p0, &p);

            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, update_buffer : Vec<VisualizerUpdate>) {

        self.since_last = self.since_last + 1;

        // Only take last for now
        for update in update_buffer {
            match update.update {
                Intensity(x) => {
                    self.since_last = 0;
                    self.last_trigger = update.time;
                    self.last_trigger_value = x;
                }
                Level(x) => {
                    let y = x.abs();
                    println!("Level {}", y);
                    self.level_prev = self.level;
                    self.level = y;
                    //self.level_beams = (self.level * 50.0).floor() as usize;
                    self.level_beams = 10;
                }
                _ => {}
            }
            self.level_angle_prev = self.level_angle;
            self.level_angle += self.level * 0.04 ;//+ self.last_trigger_value.abs() * 0.04;
            if (self.level_angle > 2.0 * 3.141) {
                self.level_angle -= 2.0 * 3.141;
            }
        }

        // Only update if song is playing
        let _ = self.start_time.elapsed().map(|current_time| {

            // 50 milliseconds
            let epilepsy_preventation_duration = Duration::new(0, 50_000_000);

            let since_trigger = diff_durs(&current_time, &self.last_trigger);

            self.on = since_trigger < epilepsy_preventation_duration;
        });

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

pub fn run(start_time : SystemTime, rx : Receiver<VisualizerUpdate>) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V4_3;

    let mut window : Window = WindowSettings::new("Simon", [800, 600])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let c = Colored::new(opengl.to_glsl());
    let t = Textured::new(opengl.to_glsl());

    let mut gl_graphics = GlGraphics::from_colored_textured(c, t);

    let mut visuals = CircleVisuals::new(start_time);
    let mut prev_time = SystemTime::now();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => {

                // Calculate fps
                let dt = prev_time.elapsed().unwrap();
                prev_time = SystemTime::now();
                let fps = 1000_000_000.0 / (dt.subsec_nanos() as f64);

                visuals.render(fps, &mut gl_graphics, &r);
            }
            Input::Update(u) => {

                // Get all the pending updates from the receiver and buffer into list
                let update_buffer = rx.try_iter().collect::<Vec<VisualizerUpdate>>();

                visuals.update(&u, update_buffer);
            }
            Input::Press(i) => {
                // Ignore for now
            }
            _ => {}
        }
    }

}
