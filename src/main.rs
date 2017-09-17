extern crate hound;

mod audio;
mod common;
mod graphics;

use std::env;
use std::time::{Duration, SystemTime};
use std::thread;
use std::thread::sleep;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::fs::File;
use std::path::Path;

use audio::run_audio;
use common::VisualizerUpdate;
use graphics::run as run_visualizer;
use hound::WavReader;


fn main() {

    // Load wav from first arg
    let arg = match env::args().nth(1) {
        Some(x) => x,
        None => {println!("Usage: simon.exe music.wav\nOr: cargo run -- music.wav"); return;},
    };

    let path = Path::new(&arg);

    let file = File::open(&path).unwrap();

    let music_start_time = SystemTime::now();
    let x = audio::mp3::run_audio(file, music_start_time);
    audio::mp3::test(x);
    /*
    let countdown = env::args().nth(2).and_then(|x| {
        x.parse::<u64>().ok()
    }).unwrap_or(7);

    let reader = WavReader::open(&arg).unwrap();
    let sample_time = 0.25;

    // Create a transmitter and receiver for updates
    let (tx, rx) : (Sender<VisualizerUpdate>, Receiver<VisualizerUpdate>)= channel();

    // Countdown allowing you to press play
    let countdown_dur = Duration::new(countdown, 0);
    // Program has headstart of a quater of a second
    let headstart = Duration::from_millis((1000.0 * sample_time) as u64);

    let music_start_time = SystemTime::now() + countdown_dur + headstart;

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rx);
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

    run_audio(reader, tx, sample_time, music_start_time);
    */
}
