use std::sync::mpsc::{Receiver, Sender};

// use interpret
use common::*;


// used to map inputs to a single graphic object
pub struct Mapper {
    input_audio: Vec<AudioType>,
    effect_gen: Fn<Vec<f64>, VisualEffect>
}

impl Mapper {
    pub fn new(input_audio: Vec<AudioType>, effect_gen: Fn<&Vec<f64>, VisualEffect>) Self {
        Mapper {
            input_audio: input_audio,
            effect_gen: effect_gen
        }
    }

    fn generate(&self, &inputs AudioPacket) GraphicsPacket {
        /*let mut args: Vec<f64> = Vec::new();
        for input in self.input_audio {
            args.push(inputs.audio[input]);
        }*/
        let args = self.input_audio.iter()
                       .map(|&x| inputs.audio.get(x).unwrap())
                       .collect::<Vec<f64>>();

        GraphicsPacket {
            effect: self.effect_gen(args),
            time: inputs.time
        }
    }
}


fn run_map(audio_rx: Receiver<AudioPacket>, graphics_tx: Sender<GraphicsPacket>, &mappers: Vec<Mapper>) {
    // pull audio data
    loop {
        let audio_in = audio_rx.recv();

        mappers.iter()
               .map(|&x| graphics_tx.send(x.generate(audio_in)).unwrap());

    }
}
