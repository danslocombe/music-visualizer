use std::time::Duration;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
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

pub struct GraphicsPacket {
    pub effect_args: Vec<Vec<f64>>,
    pub time: Duration
}
