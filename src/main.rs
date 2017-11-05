#[macro_use]
extern crate nom;
extern crate hound;
extern crate notify;
extern crate rodio;

mod audio;
mod common;
mod expression;
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

    let (visuals,bg_mapper,mappers) = parse_from_file(&script_arg);

    let song_path = Path::new(&music_arg);

    let music_start_time = SystemTime::now();
    let song = match audio::make_song(&song_path, music_start_time) {
        Some(x) => x,
        None => {::std::process::exit(1);},
    };

    let countdown = env::args().nth(3).and_then(|x| {
        x.parse::<u64>().ok()
    }).unwrap_or(0);

    // Create a transmitter and receiver for updates
    let (txa, rxa) : (Sender<AudioPacket>, Receiver<AudioPacket>) = channel();
    let (txg, rxg) : (Sender<GraphicsPacket>, Receiver<GraphicsPacket>) = channel();

    let parser_txa = txa.clone();

    // Program has headstart of a quarter of a second
    let sample_time = 0.25;

    // TODO: is this still necessary?
    let music_start_time = SystemTime::now();

    // Start the mapper
    thread::spawn(move || {
        run_map(rxa, txg, bg_mapper, mappers);
    });

    // set up watcher for file refresh
    thread::spawn(move || {
        watch_script(script_arg.as_str(), parser_txa);
    });

    // Start the graphics
    thread::spawn(move || {
        run_visualizer(music_start_time, rxg, visuals);
    });

    // start the audio analysis and playback
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
                    let (new_visuals, new_bg_mapper, new_mappers) = parse_from_file(&script_path);
                    let update = AudioPacket::Refresh(DeviceStructs {
                        bg_mapper: new_bg_mapper,
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
