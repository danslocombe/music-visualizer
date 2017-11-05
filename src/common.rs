use std::time::Duration;
use std::collections::HashMap;

use mapper::Mapper;
use graphics::ActiveEffects;

// audio outputs
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AudioType {
    Impulse,
    Level,
    //HighFrequency,
    //MidFrequency,
    //LowFrequency,
    // and many more
}

// variable arguments for visualizers
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum GArg {
    Size,
    Width,
    R,
    G,
    B,
    Trans,
    Count,
    X,
    Y,
    // Add H/S/V, speed/decay
}

// packets of data passed between threads

pub enum AudioPacket {
    Update(AudioUpdate),
    Refresh(DeviceStructs),
}

pub struct AudioUpdate {
    pub audio: HashMap<AudioType, f64>,
    pub time: Duration
}

pub struct DeviceStructs {
    pub bg_mapper: Mapper,
    pub mappers: Vec<Mapper>,
    pub visuals: ActiveEffects,
}

pub enum GraphicsPacket {
    Update(GraphicsUpdate),
    Refresh(ActiveEffects),
}

pub struct GraphicsUpdate {
    pub bg_args: Vec<(GArg, f64)>,
    pub effect_args: Vec<Vec<(GArg, f64)>>,
    pub time: Duration
}

impl GraphicsUpdate {
    pub fn new_empty(len: usize) -> Self {
        GraphicsUpdate {
            bg_args: Vec::new(),
            effect_args: vec![Vec::new(); len],
            time: Duration::new(0,0),
        }
    }
}
