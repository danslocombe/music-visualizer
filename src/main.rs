#[macro_use]
extern crate nom;

extern crate hound;

mod audio;
mod common;
mod graphics;
mod mapper;
mod parser;

use std::env;
use std::time::{Duration, SystemTime};
use std::thread;
use std::thread::sleep;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::path::Path;

use audio::run_audio;
use common::*;
use mapper::run as run_map;
use parser::parse_from_file;
use graphics::{run as run_visualizer, ActiveEffects};


fn main() {

    // Load music file and script
    let (music_arg, script_arg) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(x), Some(y)) => (x, y),
        _ => {println!("Usage: simon.exe music.wav script\nOr: cargo run -- music.wav script"); return;},
    };

    let (visuals,mappers) = parse_from_file(&script_arg);
    let effects = ActiveEffects {effects: visuals};

    let path = Path::new(&music_arg);

    let music_start_time = SystemTime::now();
    let song = match audio::make_song(&path, music_start_time) {
        Some(x) => x,
        None => {::std::process::exit(1);},
    };

    let countdown = env::args().nth(3).and_then(|x| {
        x.parse::<u64>().ok()
    }).unwrap_or(7);

    // Create a transmitter and receiver for updates
    let (txa, rxa) : (Sender<AudioPacket>, Receiver<AudioPacket>) = channel();
    let (txg, rxg) : (Sender<GraphicsPacket>, Receiver<GraphicsPacket>) = channel();

    // Countdown allowing you to press play
    let countdown_dur = Duration::new(countdown, 0);
    // Program has headstart of a quarter of a second
    let sample_time = 0.25;
    let headstart = Duration::from_millis((1000.0 * sample_time) as u64);

    let music_start_time = SystemTime::now() + countdown_dur + headstart;

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rxg, effects);
    });

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
