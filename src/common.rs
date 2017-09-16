use std::time::Duration;
use std::collections::HashMap;

pub enum AudioType {
    Intensity,
    HighFrequency,
    LowFrequency,
    // and many more
}

/*pub struct VisualizerUpdate {
    pub update : UpdateType,
    pub time : Duration
}*/

pub struct AudioPacket {
    pub audio: HashMap<AudioType, f64>,
    pub time: Duration
}

// graphics packet

pub enum VisualEffect {
    Circles(f64),
    Dots(f64, f64),
}

pub struct GraphicsPacket {
    pub effect: VisualEffect,
    pub time: Duration
}
