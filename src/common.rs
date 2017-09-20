use std::time::Duration;
use std::collections::HashMap;

// audio outputs
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AudioType {
    Impulse,
    Level,
    HighFrequency,
    LowFrequency,
    // and many more
}

// variable arguments for visualizers
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum GArg {
    Size,
    R,
    G,
    B,
    Scale,
    Count,
}

// packets of data passed between threads

pub struct AudioPacket {
    pub audio: HashMap<AudioType, f64>,
    pub time: Duration
}

pub struct GraphicsPacket {
    pub effect_args: Vec<Vec<(GArg, f64)>>,
    pub time: Duration
}
