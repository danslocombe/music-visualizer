use std::sync::mpsc::{Receiver, Sender};

// use interpret
use common::*;

// used to select between constant and variable opts
pub enum AudioOption {
    Const(f64),
    Var(AudioType)
}


// used to map inputs to a single graphic object
pub struct Mapper {
    pub input_audio: Vec<(AudioOption, GArg)>,
}

impl Mapper {
    pub fn new(input_audio: Vec<(AudioOption, GArg)>) -> Self {
        Mapper {
            input_audio: input_audio,
        }
    }

    fn generate(&self, inputs: &AudioPacket) -> Vec<(GArg, f64)> {
        /*let mut args: Vec<f64> = Vec::new();
        for input in self.input_audio {
            args.push(inputs.audio[input]);
        }*/
        // let args =...
        self.input_audio.iter()
            .map(|&(ref o,ref a)| {
                match o {
                    &AudioOption::Var(ref v) => (a.clone(),inputs.audio.get(&v).unwrap().clone()),
                    &AudioOption::Const(ref x) => (a.clone(),x.clone())
                }
            })
            .collect::<Vec<(GArg, f64)>>()

        //self.effect_gen(args)
    }
}

/*fn match_args(arg: &(AudioOption, GArg)) -> (GArg, f64) {
    let ref o,ref a = arg;
    match o {
        &AudioOption::Var
    }
}*/

pub fn run_map(audio_rx: Receiver<AudioPacket>, graphics_tx: Sender<GraphicsPacket>, mappers: &Vec<Mapper>) {
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
