use graphics::*;
use graphics::geom_visuals;

pub fn new_visualizer(name: &str) -> Box<Visualization> {
    let vis = String::from(name).to_lowercase();
    match name {
        "circles" => Box::new(geom_visuals::CircleVisuals::new()),
        "dots" => Box::new(geom_visuals::DotsVisuals::new()),
        x => panic!("Visual function not recognised: {}", x)
    }
}
