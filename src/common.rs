use std::time::Duration;
use std::collections::HashMap;

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

// expressions for audio input
#[derive(Clone,Debug)]
pub enum Expr {
    Const(f64),
    Var(AudioType),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn calculate(self, vars: &HashMap<AudioType,f64>) -> f64 {
        match self {
            Expr::Var(v) => vars.get(&v).unwrap().clone(),
            Expr::Const(x) => x,
            Expr::Add(a,b) => a.calculate(&vars) + b.calculate(&vars),
            Expr::Sub(a,b) => a.calculate(&vars) - b.calculate(&vars),
            Expr::Mul(a,b) => a.calculate(&vars) * b.calculate(&vars),
            Expr::Div(a,b) => a.calculate(&vars) / b.calculate(&vars),
        }
    }
}

// variable arguments for visualizers
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum GArg {
    Size,
    R,
    G,
    B,
    Count,
    X,
    Y,
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
