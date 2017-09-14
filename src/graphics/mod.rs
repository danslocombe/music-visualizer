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

struct Visuals {
    start_time : SystemTime,
    last_trigger : Duration,
    last_trigger_value : f64,
    on : bool,
    gl_graphics : GlGraphics,
}

impl Visuals {

    fn new(start_time : SystemTime, gl_graphics: GlGraphics) -> Self {
        Visuals {
            start_time : start_time,
            last_trigger : Duration::new(0, 0),
            last_trigger_value : 0.0,
            on : false,
            gl_graphics : gl_graphics
        }

    }

    fn render(&mut self, fps: f64, args: &RenderArgs) {
        use graphics::*;
        use graphics::graphics::clear;
        let on = self.on;
        let ltv = self.last_trigger_value;
        self.gl_graphics.draw(args.viewport(), move |_, gl| {
            let color = if on {
                color_from_val(ltv)
            }
            else {
                BLACK
            };

            clear(color, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, mut update_buffer : Vec<VisualizerUpdate>) {

        // Only take last for now
        for update in update_buffer {
            match update.update {
                Intensity(x) => {
                    println!("{}", x);
                    self.last_trigger = update.time;
                    self.last_trigger_value = x;
                }
                _ => {}
            }
        }

        // Only if song is playing
        self.start_time.elapsed().map(|current_time| {

            // 50 milliseconds
            let epilepsy_preventation_duration = Duration::new(0, 50_000_000);

            let since_trigger = diff_durs(&current_time, &self.last_trigger);

            // println!("{:?}", since_trigger);

            self.on = since_trigger < epilepsy_preventation_duration;
        });

    }
}

fn diff_durs(x : &Duration, y : &Duration) -> Duration {
    let (greater, lesser) = if x > y {
        (x, y)
    }
    else {
        (y, x)
    };
    *greater - *lesser
}

pub fn run(start_time : SystemTime, rx : Receiver<VisualizerUpdate>) {
    // Try a different version if this doesn't work
    let opengl = OpenGL::V4_3;

    let mut window : Window = WindowSettings::new("Simon", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let c = Colored::new(opengl.to_glsl());
    let t = Textured::new(opengl.to_glsl());

    let gl_graphics = GlGraphics::from_colored_textured(c, t);

    let mut visuals = Visuals::new(start_time, gl_graphics);
    let mut prev_time = SystemTime::now();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(r) => {
                let dt = prev_time.elapsed().unwrap();
                prev_time = SystemTime::now();
                let fps = 1000_000_000.0 / (dt.subsec_nanos() as f64);
                visuals.render(fps, &r);
            }
            Input::Update(u) => {
                // Get all the pending updates from the receiver
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
