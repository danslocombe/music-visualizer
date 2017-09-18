

// use interpret

use common::*;
use std::sync::mpsc::{Receiver, Sender};

// used to select between constant and variable opts
pub enum AudioOption {
    Const(f64),
    Var(AudioType),
}


// used to map inputs to a single graphic object
pub struct Mapper {
    pub input_audio: Vec<AudioOption>,
    //effect_gen: Box<(FnOnce(Vec<f64>) -> Vec<f64>)>
}

impl Mapper {
    pub fn new(input_audio: Vec<AudioOption> /*, effect_gen: Box<(FnOnce(Vec<f64>) -> Vec<f64>)>*/)
        -> Self {
        Mapper {
            input_audio: input_audio,
            //effect_gen: effect_gen
        }
    }

    fn generate(&self, inputs: &AudioPacket) -> Vec<f64> {
        /*let mut args: Vec<f64> = Vec::new();
        for input in self.input_audio {
            args.push(inputs.audio[input]);
        }*/
        // let args =...
        self.input_audio
            .iter()
            .map(|o| match *o {
                AudioOption::Var(ref v) => inputs.audio[v],
                AudioOption::Const(ref x) => *x,
            })
            .collect::<Vec<f64>>()

        //self.effect_gen(args)
    }
}


pub fn run_map(
    audio_rx: &Receiver<AudioPacket>,
    graphics_tx: &Sender<GraphicsPacket>,
    mappers: &[Mapper],
) {
    while let Ok(audio_in) = audio_rx.recv() {

        let effect_args = mappers
            .iter()
            .map(|m| m.generate(&audio_in))
            .collect::<Vec<Vec<f64>>>();

        let packet = GraphicsPacket {
            effect_args: effect_args,
            time: audio_in.time,
        };

        graphics_tx.send(packet).unwrap();
    }
}
