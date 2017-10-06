use graphics::*;
use graphics::geom_visuals;

pub fn new_visualizer(name: &str) -> Box<Visualization> {
    let vis = name.to_lowercase();
    match vis.as_str() {
        "circles" => Box::new(geom_visuals::CircleVisuals::new()),
        "dots" => Box::new(geom_visuals::DotsVisuals::new()),
        "bar" => Box::new(geom_visuals::BarVisuals::new()),
        x => panic!("Visual function not recognised: {}", x)
    }
}
