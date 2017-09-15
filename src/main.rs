extern crate hound;

mod audio;
mod common;
mod graphics;

use std::env;
use std::time::{Duration, SystemTime};
use std::thread;
use std::thread::sleep;
use std::sync::mpsc::{Receiver, Sender, channel};

use audio::run_audio;
use common::VisualizerUpdate;
use graphics::run as run_visualizer;
use hound::WavReader;


fn main() {

    // Load wav from first arg
    let arg = match env::args().nth(1) {
        Some(x) => x,
        None => {println!("Usage: simon.exe music.wav"); return;},
    };

    let reader = WavReader::open(&arg).unwrap();
    let sample_time = 0.25;

    // Create a transmitter and receiver for updates
    let (tx, rx) : (Sender<VisualizerUpdate>, Receiver<VisualizerUpdate>)= channel();

    // Countdown allowing you to press play
    let countdown_dur = Duration::new(7, 0);
    // Program has headstart of a quater of a second
    let headstart = Duration::from_millis((1000.0 * sample_time) as u64);

    let music_start_time = SystemTime::now() + countdown_dur + headstart;

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rx);
    });

    thread::spawn(move || {
        sleep(headstart);
        for i in 0..7 {
            let ii = 7 - i;
            println!("{}", ii); 
            sleep(Duration::new(1, 0));
        }
        println!("Play!"); 
    });

    // Doesn't sleep for headstart duration, but sleeps for countdown
    sleep(countdown_dur);

    run_audio(reader, tx, sample_time, music_start_time);
}
