use graphics::*;
use graphics::geom_visuals;
use std::time::SystemTime; // sort this out

pub fn new_visualizer(name: &str) -> Box<Visualization + Send> {
    let vis = String::from(name).to_lowercase();
    match name {
        "circles" => Box::new(geom_visuals::CircleVisuals::new(SystemTime::now())),
        "dots" => Box::new(geom_visuals::DotsVisuals::new()),
        _ => panic!("Visual function not recognised.")
    }
}
