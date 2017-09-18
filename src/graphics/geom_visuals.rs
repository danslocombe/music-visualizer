use graphics::{Color, Visualization};
use graphics::opengl_graphics::GlGraphics;
use graphics::piston::input::RenderArgs;
use std::time::{Duration, SystemTime};

const TWO_PI: f64 = 6.282;

// The difference between two durations
fn diff_durs(x: &Duration, y: &Duration) -> Duration {
    let (greater, lesser) = if x > y { (x, y) } else { (y, x) };
    *greater - *lesser
}

struct Point {
    x: f64,
    y: f64,
}

// Taken from another project
fn line_points(
    gl: &mut GlGraphics,
    args: &RenderArgs,
    color: Color,
    width: f64,
    p1: &Point,
    p2: &Point,
) {
    use graphics::*;

    let camera_scale = f64::from(args.width / 2);

    let (x_mid, y_mid) =
        (f64::from(args.width / 2), f64::from(args.height / 2));

    let Point { x: x1, y: y1 } = *p1;
    let Point { x: x2, y: y2 } = *p2;

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

pub struct CircleVisuals {
    start_time: SystemTime,
    last_trigger: Duration,
    last_trigger_value: f64,
    since_last: u32,
    on: bool,
    color: Color,
    multiplier: f64,
}

impl CircleVisuals {
    pub fn new(start_time: SystemTime, color: Color, mult: f64) -> Self {
        CircleVisuals {
            start_time: start_time,
            since_last: 0,
            last_trigger: Duration::new(0, 0),
            last_trigger_value: 0.0,
            on: false,
            color: color,
            multiplier: mult,
        }
    }
}

impl Visualization for CircleVisuals {
    fn render(
        &self,
        fps: f64,
        gl_graphics: &mut GlGraphics,
        args: &RenderArgs,
    ) {
        gl_graphics.draw(args.viewport(), |_, gl| {
            // Circle precision
            let prec: i32 = 32;
            let prec_d = f64::from(prec);

            // Circle radius
            let r_mult = self.last_trigger_value.abs() * self.multiplier.abs();

            let r = if self.on {
                r_mult
            } else {
                r_mult / f64::from(self.since_last)
            } * 2.0;

            // Draw a circle of radius r
            for i in 0..prec {
                let i_d = f64::from(i);
                let i1_d = f64::from(i + 1);
                let p1 = Point {
                    x: r * (TWO_PI * i_d / prec_d).cos(),
                    y: r * (TWO_PI * i_d / prec_d).sin(),
                };
                let p2 = Point {
                    x: r * (TWO_PI * i1_d / prec_d).cos(),
                    y: r * (TWO_PI * i1_d / prec_d).sin(),
                };
                line_points(gl, args, self.color, 1.0, &p1, &p2);
            }
        });
    }

    fn update(&mut self, args: &[f64], args_time: Duration) {
        self.since_last += 1;

        if !args.is_empty() {
            if (args[0]) > 0.0 {
                self.since_last = 0;
                self.last_trigger = args_time;
                self.last_trigger_value = args[0];
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

pub struct DotsVisuals {
    since_last: usize,
    x: f64,
    x_prev: f64,
    angle: f64,
    angle_prev: f64,
    dots: usize,
    color: Color,
    multiplier: f64,
}

impl DotsVisuals {
    pub fn new(color: Color, dots: usize, mult: f64) -> Self {
        DotsVisuals {
            since_last: 0,
            x: 0.0,
            x_prev: 0.0,
            angle: 0.0,
            angle_prev: 0.0,
            dots: 32,
            color: color,
            multiplier: mult,
        }
    }
}

impl Visualization for DotsVisuals {
    fn render(
        &self,
        fps: f64,
        gl_graphics: &mut GlGraphics,
        args: &RenderArgs,
    ) {
        gl_graphics.draw(args.viewport(), |_, gl| {

            // Draw a circle of radius r
            for i in 0..self.dots {
                let r0 = self.x_prev * self.multiplier;
                let theta0 = self.angle_prev +
                    (i as f64) * TWO_PI / (self.dots as f64);
                let r1 = self.x * self.multiplier;
                let theta = self.angle +
                    (i as f64) * TWO_PI / (self.dots as f64);


                let p0 = Point {
                    x: r0 * (theta0).cos(),
                    y: r0 * (theta0).sin(),
                };
                let p1 = Point {
                    x: r1 * theta.cos(),
                    y: r1 * theta.sin(),
                };

                line_points(gl, args, self.color, 1.0, &p0, &p1);
            }
        });
    }

    fn update(&mut self, args: &[f64], args_time: Duration) {
        self.since_last += 1;

        if !args.is_empty() {
            self.x_prev = self.x;
            self.x = args[0];
        }

        self.angle_prev = self.angle;
        self.angle += self.x * 0.04;
        if (self.angle > TWO_PI) {
            self.angle -= TWO_PI;
        }
    }
}
