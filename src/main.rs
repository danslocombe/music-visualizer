extern crate hound;

mod audio;
mod common;
mod graphics;
mod module_map;

use std::env;
use std::time::{Duration, SystemTime};
use std::thread;
use std::thread::sleep;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::fs::File;
use std::path::Path;

use audio::run_audio;
use common::*;
use module_map::*;
use graphics::run as run_visualizer;
use hound::WavReader;


fn main() {

    // Load wav from first arg
    let arg = match env::args().nth(1) {
        Some(x) => x,
        None => {println!("Usage: simon.exe music.wav\nOr: cargo run -- music.wav"); return;},
    };

    let path = Path::new(&arg);

    let music_start_time = SystemTime::now();
    let song = match audio::make_song(&path, music_start_time) {
        Some(x) => x,
        None => {::std::process::exit(1);},
    };

    let countdown = env::args().nth(2).and_then(|x| {
        x.parse::<u64>().ok()
    }).unwrap_or(7);

    //let reader = WavReader::open(&arg).unwrap();
    let sample_time = 0.25;

    // Create a transmitter and receiver for updates
    let (txa, rxa) : (Sender<AudioPacket>, Receiver<AudioPacket>) = channel();
    let (txg, rxg) : (Sender<GraphicsPacket>, Receiver<GraphicsPacket>) = channel();

    // Countdown allowing you to press play
    let countdown_dur = Duration::new(countdown, 0);
    // Program has headstart of a quater of a second
    let headstart = Duration::from_millis((1000.0 * sample_time) as u64);

    let music_start_time = SystemTime::now() + countdown_dur + headstart;

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rxg);
    });

    // Temp: generate mapper
    /*let mapper = Mapper {
        input_audio: vec![AudioOption::Var(AudioType::Intensity)],
        //effect_gen: Box::new(move |v: Vec<f64>| v),
    };*/

    let mappers: Vec<Mapper> = vec![
        Mapper { input_audio: vec![AudioOption::Var(AudioType::Impulse)] },
        Mapper { input_audio: vec![AudioOption::Var(AudioType::Level)] },
    ];

    // Start the mapper
    thread::spawn(move || {
        run_map(rxa, txg, &mappers);
    });

    thread::spawn(move || {
        sleep(headstart);
        for i in 0..countdown {
            let ii = countdown - i;
            println!("{}", ii); 
            sleep(Duration::new(1, 0));
        }
        println!("Play!"); 
    });

    // Doesn't sleep for headstart duration, but sleeps for countdown
    sleep(countdown_dur);

    run_audio(song, txa, sample_time, music_start_time);
}
