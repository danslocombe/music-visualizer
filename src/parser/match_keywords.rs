use std::str;
use common::{AudioType, GArg, AudioOption};

pub fn check_garg_name(i: &[u8]) -> Result<GArg, String> {
    let identifier = str::from_utf8(i).unwrap().to_lowercase();
    match identifier.as_str() {
        "size" => Ok(GArg::Size),
        "r" => Ok(GArg::R),
        "red" => Ok(GArg::R),
        "g" => Ok(GArg::G),
        "green" => Ok(GArg::G),
        "b" => Ok(GArg::B),
        "blue" => Ok(GArg::B),
        "scale" => Ok(GArg::Scale),
        "count" => Ok(GArg::Count),
        x => Err(format!("Invalid graphic argument specified: {}", x)),
    }
}


pub fn check_audio_name(i: &[u8]) -> Result<AudioOption, String> {
    let identifier = str::from_utf8(i).unwrap().to_lowercase();
    match identifier.as_str() {
        "impulse" => Ok(AudioOption::Var(AudioType::Impulse)),
        "level" => Ok(AudioOption::Var(AudioType::Level)),
        x => Err(format!("Invalid audio input specified: {}", x)),
    }
}
