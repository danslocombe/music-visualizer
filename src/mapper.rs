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

    fn generate(&self, inputs: &AudioUpdate) -> Vec<(GArg, f64)> {
        self.input_audio.iter()
            .cloned()
            .map(|(o, a)| {
                (a, o.calculate(&inputs.audio))
            })
            .collect::<Vec<(GArg, f64)>>()
    }
}

pub fn run(audio_rx: Receiver<AudioPacket>, graphics_tx: Sender<GraphicsPacket>, init_mappers: Vec<Mapper>) {
    let mut mappers = init_mappers;

    while let Ok(audio_in) = audio_rx.recv(){
        let packet = match audio_in {
            AudioPacket::Update(data) => {
                let effect_args = mappers.iter()
                                         .map(|m| m.generate(&data))
                                         .collect::<Vec<Vec<(GArg, f64)>>>();

                GraphicsPacket::Update(GraphicsUpdate {
                    effect_args: effect_args,
                    time: data.time
                })
            }
            AudioPacket::Refresh(new_structs) => {
                mappers = new_structs.mappers;

                GraphicsPacket::Refresh(new_structs.visuals)
            }
        };

        graphics_tx.send(packet).unwrap();
    }
}
