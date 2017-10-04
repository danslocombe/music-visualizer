use std::sync::mpsc::{Receiver, Sender};

// use interpret
use common::*;


// used to map inputs to a single graphic object
pub struct Mapper {
    pub input_audio: Vec<(Expr, GArg)>,
}

impl Mapper {
    pub fn new(input_audio: Vec<(Expr, GArg)>) -> Self {
        Mapper {
            input_audio: input_audio,
        }
    }

    fn generate(&self, inputs: &AudioPacket) -> Vec<(GArg, f64)> {
        self.input_audio.iter()
            .cloned()
            .map(|(o, a)| {
                (a, o.calculate(&inputs.audio))
            })
            .collect::<Vec<(GArg, f64)>>()
    }
}

pub fn run(audio_rx: Receiver<AudioPacket>, graphics_tx: Sender<GraphicsPacket>, mappers: &Vec<Mapper>) {
    while let Ok(audio_in) = audio_rx.recv(){

        let effect_args = mappers.iter()
                                 .map(|m| m.generate(&audio_in))
                                 .collect::<Vec<Vec<(GArg, f64)>>>();

        let packet = GraphicsPacket {
                        effect_args: effect_args,
                        time: audio_in.time
                     };

        graphics_tx.send(packet).unwrap();
    }
}
