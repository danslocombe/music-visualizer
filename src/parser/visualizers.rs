use graphics::*;
use graphics::backgrounds;
use graphics::geom_visuals;

pub fn new_background(name: &str) -> Box<Background> {
    let bg = name.to_lowercase();
    match bg.as_str() {
        "fill" => Box::new(backgrounds::SolidColor::new()),
        "solid" => Box::new(backgrounds::SolidColor::new()),
        x => panic!("Background not recognised: {}", x)
    }
}

pub fn new_visualizer(name: &str) -> Box<Visualization> {
    let vis = name.to_lowercase();
    match vis.as_str() {
        "circles" => Box::new(geom_visuals::CircleVisuals::new()),
        "dots" => Box::new(geom_visuals::DotsVisuals::new()),
        "bar" => Box::new(geom_visuals::BarVisuals::new()),
        "spiky" => Box::new(geom_visuals::SpikyVisuals::new()),
        x => panic!("Visual function not recognised: {}", x)
    }
}
