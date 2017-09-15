use std::time::Duration;

pub enum UpdateType {
    Intensity(f64),
}

pub struct VisualizerUpdate {
    pub update : UpdateType,
    pub time : Duration
}

