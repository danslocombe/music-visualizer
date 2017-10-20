use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use graphics::opengl_graphics::GlGraphics;
use graphics::piston::input::RenderArgs;
use graphics::{Color, Visualization};
use common::GArg;

const TWO_PI : f64 = 6.282;

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

macro_rules! make_map {
    ( $($k:expr , $v:expr);* ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

#[inline]
fn cons_color(vars: &HashMap<GArg, f64>) -> Color {
    let r = vars.get(&GArg::R).unwrap().clone();
    let g = vars.get(&GArg::G).unwrap().clone();
    let b = vars.get(&GArg::B).unwrap().clone();
    let transparency = vars.get(&GArg::Trans).unwrap().clone();
    [r as f32, g as f32, b as f32, transparency as f32]
}

#[inline]
fn arg(vars: &HashMap<GArg, f64>, arg: GArg) -> f64 {
    vars.get(&arg).unwrap().clone()
}

pub struct CircleVisuals {
    start_time : SystemTime,
    last_trigger : Duration,
    since_last : u32,
    on : bool,
    vars : HashMap<GArg, f64>
}

impl CircleVisuals {
    pub fn new() -> Self {
        let vars = make_map![GArg::Size,0.0;
                             GArg::R,1.0;GArg::G,1.0;GArg::B,1.0;GArg::Trans,1.0;
                             GArg::X,0.5;GArg::Y,0.5];
        CircleVisuals {
            start_time : SystemTime::now(),
            since_last : 0,
            last_trigger : Duration::new(0, 0),
            on : false,
            vars : vars
        }
    }

    pub fn newv(vars: &[(GArg, f64)]) -> Self {
        let mut v = Self::new();
        for (a,f) in vars.iter().cloned() {
            v.vars.insert(a,f);
        };
        v
    }
}

impl Visualization for CircleVisuals {

    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl_graphics.draw(args.viewport(), |_, gl| {
            // Circle precision
            let prec : i32 = 32;
            let prec_d = prec as f64;

            // Circle centre
            let x_cent = (arg(&self.vars,GArg::X) * 2.0) - 1.0;
            let y_cent = (arg(&self.vars,GArg::Y) * 2.0) - 1.0;

            // Circle radius
            let r_mult = arg(&self.vars,GArg::Size).abs();

            let r = if self.on {
                r_mult
            }
            else {
                r_mult / (self.since_last as f64)
            } * 2.0;

            let color = cons_color(&self.vars);
            // Draw a circle of radius r
            for i in 0..prec {
                let i_d = i as f64;
                let i1_d = (i + 1) as f64;
                let p1 = Point{
                    x : (r * (TWO_PI * i_d / prec_d).cos()) + x_cent,
                    y : (r * (TWO_PI * i_d / prec_d).sin()) + y_cent,
                };
                let p2 = Point{
                    x : (r * (TWO_PI * i1_d / prec_d).cos()) + x_cent,
                    y : (r * (TWO_PI * i1_d / prec_d).sin()) + y_cent,
                };
                line_points(gl, args, color, 1.0, &p1, &p2);
            }
        });
    }

    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration) {
        self.since_last = self.since_last + 1;

        let last_size = arg(&self.vars,GArg::Size);

        for (a, v) in args.iter().cloned() {
            self.vars.insert(a,v);
        }

        if arg(&self.vars,GArg::Size) > 0.0 {
            self.since_last = 1;
            self.last_trigger = args_time;
        }
        else {
            self.vars.insert(GArg::Size,last_size);
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

pub struct DotsVisuals {
    since_last : usize,
    size_prev : f64,
    angle : f64,
    angle_prev : f64,
    vars : HashMap<GArg, f64>
}

impl DotsVisuals {
    pub fn new() -> Self {
        let vars = make_map![GArg::Size,0.0;GArg::Count,32.0;
                             GArg::R,1.0;GArg::G,1.0;GArg::B,1.0;GArg::Trans,1.0;
                             GArg::X,0.5;GArg::Y,0.5];
        DotsVisuals {
            since_last : 0,
            size_prev : 0.0,
            angle : 0.0,
            angle_prev : 0.0,
            vars : vars
        }
    }

    pub fn newv(vars: &[(GArg, f64)]) -> Self {
        let mut v = Self::new();
        for (a,f) in vars.iter().cloned() {
            v.vars.insert(a,f);
        };
        v
    }
}

impl Visualization for DotsVisuals {

    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        gl_graphics.draw(args.viewport(), |_, gl| {
            // centre of dots
            let x_cent = (arg(&self.vars,GArg::X) * 2.0) - 1.0;
            let y_cent = (arg(&self.vars,GArg::Y) * 2.0) - 1.0;

            let color = cons_color(&self.vars);

            // Draw a circle of radius r
            let dots = arg(&self.vars,GArg::Count) as u32;
            for i in 0..dots {
                let r0 = self.size_prev;
                let theta0 = self.angle_prev + (i as f64) * TWO_PI / arg(&self.vars,GArg::Count);
                let r1 = arg(&self.vars,GArg::Size);
                let theta = self.angle + (i as f64) * TWO_PI / arg(&self.vars,GArg::Count);
            
                let p0 = Point{
                    x : (r0 * (theta0).cos()) + x_cent,
                    y : (r0 * (theta0).sin()) + y_cent,
                };
                let p1 = Point{
                    x : (r1 * theta.cos()) + x_cent,
                    y : (r1 * theta.sin()) + y_cent,
                };

                line_points(gl, args, color, 1.0, &p0, &p1);
            }
        });
    }

    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration) {
        self.since_last = self.since_last + 1;

        self.size_prev = arg(&self.vars,GArg::Size);

        for (a,v) in args.iter().cloned() {
            self.vars.insert(a,v);
        }

        self.angle_prev = self.angle;
        self.angle += arg(&self.vars,GArg::Size) * 0.04;
        if self.angle > TWO_PI {
            self.angle -= TWO_PI;
        }
    }
}

//struct 
pub struct BarVisuals {
    since_last : usize,
    size_prev : f64,
    vars : HashMap<GArg, f64>
}

impl BarVisuals {
    pub fn new() -> Self {
        let vars = make_map![GArg::Size,0.0;
                             GArg::R,1.0;GArg::G,1.0;GArg::B,1.0;GArg::Trans,1.0;
                             GArg::X,0.0;GArg::Y,1.0];
        BarVisuals {
            since_last: 0,
            size_prev: 0.0,
            vars: vars
        }
    }
}

impl Visualization for BarVisuals {
    fn render(&self, fps: f64, gl_graphics : &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        use graphics::graphics::Transformed;

        gl_graphics.draw(args.viewport(), |c, gl| {
            let color = cons_color(&self.vars);

            let rect_width = (args.width / 10) as f64;
            let rect_height = arg(&self.vars,GArg::Size)/* * ((args.height / 2) as f64)*/;

            let x = arg(&self.vars,GArg::X) * (args.width as f64);
            let y = arg(&self.vars,GArg::Y) * (args.height as f64);

            let transform = c.transform.trans(x, y)
                                       .flip_v()
                                       .scale(1.0, rect_height);

            let shape = graphics::rectangle::square(0.0, 0.0, rect_width);

            graphics::rectangle(color, shape, transform, gl);
        });
    }

    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration) {
        self.since_last = self.since_last + 1;

        self.size_prev = arg(&self.vars,GArg::Size);
        
        for (a,v) in args.iter().cloned() {
            self.vars.insert(a,v);
        }
    }
}

/*pub struct Visuals {
    start_time : SystemTime,
    last_trigger : Duration,
    since_last : u32,
    on : bool,
    vars : HashMap<GArg, f64>
}

impl CircleVisuals {
    pub fn new() -> Self {
        let vars = make_map![GArg::Size,0.0;GArg::R,1.0;GArg::G,1.0;GArg::B,1.0;
                             GArg::X,0.5;GArg::Y,0.5];
        CircleVisuals {
            start_time : SystemTime::now(),
            since_last : 0,
            last_trigger : Duration::new(0, 0),
            on : false,
            vars : vars
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

            // Circle centre
            let x_cent = (arg(&self.vars,GArg::X) * 2.0) - 1.0;
            let y_cent = (arg(&self.vars,GArg::Y) * 2.0) - 1.0;

            // Circle radius
            let r_mult = arg(&self.vars,GArg::Size).abs();

            let r = if self.on {
                r_mult
            }
            else {
                r_mult / (self.since_last as f64)
            } * 2.0;

            let color = cons_color(&self.vars);
            // Draw a circle of radius r
            for i in 0..prec {
                let i_d = i as f64;
                let i1_d = (i + 1) as f64;
                let p1 = Point{
                    x : (r * (TWO_PI * i_d / prec_d).cos()) + x_cent,
                    y : (r * (TWO_PI * i_d / prec_d).sin()) + y_cent,
                };
                let p2 = Point{
                    x : (r * (TWO_PI * i1_d / prec_d).cos()) + x_cent,
                    y : (r * (TWO_PI * i1_d / prec_d).sin()) + y_cent,
                };
                line_points(gl, args, color, 1.0, &p1, &p2);
            }
        });
    }

    fn update(&mut self, args: &[(GArg, f64)], args_time: Duration) {
        self.since_last = self.since_last + 1;

        let last_size = arg(&self.vars,GArg::Size);

        for (a, v) in args.iter().cloned() {
            self.vars.insert(a,v);
        }

        if arg(&self.vars,GArg::Size) > 0.0 {
            self.since_last = 1;
            self.last_trigger = args_time;
        }
        else {
            self.vars.insert(GArg::Size,last_size);
        }

        // Only update if song is playing
        let _ = self.start_time.elapsed().map(|current_time| {

            // 50 milliseconds
            let epilepsy_preventation_duration = Duration::new(0, 50_000_000);

            let since_trigger = diff_durs(&current_time, &self.last_trigger);

            self.on = since_trigger < epilepsy_preventation_duration;
        });
    }
}*/
