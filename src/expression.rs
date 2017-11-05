use common::AudioType;
use std::collections::HashMap;

// Expressions
#[derive(Clone,Debug)]
pub enum Expr {
    Const(f64),
    Var(AudioType),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    // Functions (may move these)
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Floor(Box<Expr>),
    Ceil(Box<Expr>),
}

// Evaluation
impl Expr {
    pub fn calculate(self, vars: &HashMap<AudioType,f64>) -> f64 {
        match self {
            Expr::Var(v) => vars.get(&v).unwrap().clone(),
            Expr::Const(x) => x,
            Expr::Add(a,b) => a.calculate(&vars) + b.calculate(&vars),
            Expr::Sub(a,b) => a.calculate(&vars) - b.calculate(&vars),
            Expr::Mul(a,b) => a.calculate(&vars) * b.calculate(&vars),
            Expr::Div(a,b) => a.calculate(&vars) / b.calculate(&vars),
            // Functions
            Expr::Cond(c,a,b) => if c.calculate(&vars) > 0.0 {
                                     a.calculate(&vars)
                                 } else {
                                     b.calculate(&vars)
                                 },
            Expr::Sin(x) => x.calculate(&vars).sin(),
            Expr::Cos(x) => x.calculate(&vars).cos(),
            Expr::Floor(x) => x.calculate(&vars).floor(),
            Expr::Ceil(x) => x.calculate(&vars).ceil(),
        }
    }
}
