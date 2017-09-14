use std::time::Duration;

pub enum UpdateType {
    SongStart,
    SongEnd,
    Intensity(f64),
}

pub struct VisualizerUpdate {
    pub update : UpdateType,
    pub time : Duration
}

