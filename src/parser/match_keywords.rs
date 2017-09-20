use std::str;
use common::{AudioType, GArg};
use mapper::AudioOption;

pub fn check_garg_name(i: &[u8]) -> GArg {
    GArg::Size
}


pub fn check_audio_name(i: &[u8]) -> AudioOption {
    /*let identifier = str::from_utf8(i).to_lowercase();
    match identifier {
        "impulse" => Done(l,AudioOption::Impulse),
        "level" => Done(l,AudioOption::Level),
        _ => Error(0),
    }*/
    AudioOption::Var(AudioType::Impulse)
}
