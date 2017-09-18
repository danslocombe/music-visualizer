use std::collections::HashMap;
use std::time::Duration;

#[derive(Hash, PartialEq, Eq)]
pub enum AudioType {
    Impulse,
    Level,
    HighFrequency,
    LowFrequency,
    // and many more
}

pub struct AudioPacket {
    pub audio: HashMap<AudioType, f64>,
    pub time: Duration,
}

// graphics packet

pub struct GraphicsPacket {
    pub effect_args: Vec<Vec<f64>>,
    pub time: Duration,
}
