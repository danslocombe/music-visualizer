use common::GArg;
use std::collections::HashMap;

pub type Color = [f32; 4];

fn color_from_val(x : f64) -> Color {
    let y = x as f32;
    // Set alpha to 1
    [y, y, y, 1.0]
}

#[export_macro]
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
pub fn cons_color(vars: &HashMap<GArg, f64>) -> Color {
    let r = vars.get(&GArg::R).unwrap().clone();
    let g = vars.get(&GArg::G).unwrap().clone();
    let b = vars.get(&GArg::B).unwrap().clone();
    let transparency = vars.get(&GArg::Trans).unwrap().clone();
    [r as f32, g as f32, b as f32, transparency as f32]
}

#[inline]
pub fn arg(vars: &HashMap<GArg, f64>, arg: GArg) -> f64 {
    vars.get(&arg).unwrap().clone()
}
