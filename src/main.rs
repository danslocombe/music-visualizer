#[macro_use]
extern crate nom;

extern crate hound;
extern crate notify;

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
use graphics::run as run_visualizer;

use notify::{Watcher, RecursiveMode, RecommendedWatcher, DebouncedEvent};

fn main() {

    // Load music file and script
    let (music_arg, script_arg) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(x), Some(y)) => (x, y),
        _ => {println!("Usage: audisuals.exe music.wav script\nOr: cargo run -- music.wav script"); return;},
    };
    
    //let mut script_path = env::current_dir().unwrap();
    //script_path.push(&script_arg);

    let (visuals,mappers) = parse_from_file(&script_arg);

    let song_path = Path::new(&music_arg);

    let music_start_time = SystemTime::now();
    let song = match audio::make_song(&song_path, music_start_time) {
        Some(x) => x,
        None => {::std::process::exit(1);},
    };

    let countdown = env::args().nth(3).and_then(|x| {
        x.parse::<u64>().ok()
    }).unwrap_or(7);

    // Create a transmitter and receiver for updates
    let (txa, rxa) : (Sender<AudioPacket>, Receiver<AudioPacket>) = channel();
    let (txg, rxg) : (Sender<GraphicsPacket>, Receiver<GraphicsPacket>) = channel();

    let parser_txa = txa.clone();

    // Countdown allowing you to press play
    let countdown_dur = Duration::new(countdown, 0);
    // Program has headstart of a quarter of a second
    let sample_time = 0.25;
    let headstart = Duration::from_millis((1000.0 * sample_time) as u64);

    let music_start_time = SystemTime::now() + countdown_dur + headstart;

    // Start the mapper
    thread::spawn(move || {
        run_map(rxa, txg, mappers);
    });

    // set up watcher for file refresh
    thread::spawn(move || {
        watch_script(script_arg.as_str(), parser_txa);
    });

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rxg, visuals);
    });

    // countdown...
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

    // start the audio analysis
    run_audio(song, txa, sample_time, music_start_time);
}

// watches the script for changes.
fn watch_script(script_path: &str, txa: Sender<AudioPacket>) {
    let (txf, rxf) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(txf, Duration::from_millis(1)).unwrap();

    watcher.watch(&script_path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rxf.recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(_) => {
                    let (new_visuals, new_mappers) = parse_from_file(&script_path);
                    let update = AudioPacket::Refresh(DeviceStructs {
                        mappers: new_mappers,
                        visuals: new_visuals
                    });
                    txa.send(update).unwrap();
                },
                _ => {}
            },
            Err(e) => {
                println!("Watch error: {:?}", e);
                break;
            }
        }
    }
}
