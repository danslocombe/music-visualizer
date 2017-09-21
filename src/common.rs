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

// used to select between different audio input types
#[derive(Clone,Debug)]
pub enum AudioOption {
    Const(f64),
    Var(AudioType)
    // TODO: expressions
}

// variable arguments for visualizers
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum GArg {
    Size,
    R,
    G,
    B,
    Scale,  // could be removed when expressions are added
    Count,
    // Add H/S/V, speed/decay
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
