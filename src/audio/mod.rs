use common::{AudioPacket, AudioType};
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub mod mp3;
pub mod wav;

type Sample = i32;

pub trait Song: Iterator<Item = AudioData> {
    fn sample_max_value(&self) -> u32;
}

pub struct AudioData {
    pub sample: Sample,
    pub time: Duration,
}

pub fn make_song(
    path: &Path,
    start_time: SystemTime,
) -> Option<Box<Song<Item = AudioData>>> {
    path.extension().and_then(|ext| match ext.to_str()
        .unwrap()
        .to_lowercase()
        .as_ref() {
        "wav" => {
            let file = File::open(path).unwrap();
            let wav = wav::WavSong::new(file).unwrap();
            let x: Box<Song<Item = AudioData>> = Box::new(wav);
            Some(x)
        }
        "mp3" => {
            let file = File::open(path).unwrap();
            let mp3 = mp3::Mp3Song::new(file, start_time);
            let x: Box<Song<Item = AudioData>> = Box::new(mp3);
            Some(x)
        }
        x => {
            println!("Error: unknown file extension \"{}\"", x);
            None
        }
    })
}

pub fn run_audio(
    mut song: Box<Song<Item = AudioData>>,
    tx: Sender<AudioPacket>,
    sample_time: f64,
    start_time: SystemTime,
) {

    let sample_max = song.sample_max_value();
    let mut audio_proc =
        AudioProcessor::new(tx, sample_time, start_time, sample_max);

    for data in &mut song {
        audio_proc.process_sample(data.sample, data.time);
    }
}

struct AudioProcessor {
    window: TimeWindow,
    tx: Sender<AudioPacket>,
    start_time: SystemTime,
    sample_max: u32,
    impulse_triggered: bool,
    sample_number: usize,
}

impl AudioProcessor {
    fn new(
        tx: Sender<AudioPacket>,
        sample_time: f64,
        start_time: SystemTime,
        sample_max: u32,
    ) -> Self {

        // For now we window over a half a second (Completely arbitrary)
        // This is a quater of a second forwards and backwards in time
        // Assume for now sample rate about 44k
        let window_size = 44000.0 * sample_time;
        let window = TimeWindow::new(window_size as usize);
        AudioProcessor {
            tx: tx,
            window: window,
            start_time: start_time,
            sample_number: 0,
            sample_max: sample_max,
            impulse_triggered: false,
        }
    }

    fn process_sample(&mut self, x: Sample, time: Duration) {

        // Add the new sample to the window
        self.window.step_forwards(x);

        // Check if the new sample is significant
        let sig = self.window.current_significant();

        if (sig) {
            self.impulse_triggered = true;
        }

        // Arbitrary again
        if (self.sample_number % 400 == 0) {

            let mut audio_map: HashMap<AudioType, f64> = HashMap::new();

            // If we are switiching to a new state
            let impulse_intensity = if self.impulse_triggered {
                f64::from(x) / f64::from(self.sample_max)
            } else {
                0.0
            };

            self.impulse_triggered = false;

            audio_map.insert(AudioType::Impulse, impulse_intensity);

            let maybe_time_diff = self.start_time.elapsed().ok().and_then(
                |current_songtime| time.checked_sub(current_songtime),
            );

            if let Some(time_diff) = maybe_time_diff {

                // Sleep until the point in the song where we were triggered
                sleep(time_diff);

                let i = 5.0 * self.window.std_dev() /
                    f64::from(self.sample_max);
                audio_map.insert(AudioType::Level, i);
                let update = AudioPacket {
                    time: time,
                    audio: audio_map,
                };
                try_send_update(&self.tx, update);
            }
        }
        self.sample_number += 1;
    }
}


fn try_send_update(tx: &Sender<AudioPacket>, update: AudioPacket) {

    match tx.send(update) {
        Ok(_) => { /* Sent ok */ }

        Err(_) => {
            // Other end of channel closed, so the visualizer must have
            // been closed / crashed, we treat this optimistically
            println!("Visualizer window closed, exiting..");
            ::std::process::exit(0);
        }
    }
}

// A "Time Window" focused on a particular point in an audio file
#[derive(Debug)]
struct TimeWindow {
    size: usize,

    current_sample: i64,

    // The sum of all the samples the window
    // can view
    sum: i64,

    // The sum of the difference of all the samples
    // from the mean
    dev_total: f64,

    past: LinkedList<Sample>,
    present: Sample,
    future: LinkedList<Sample>,
}

impl TimeWindow {
    fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Tried to create a TimeWindow containing only the present!");
        }
        let mut past = LinkedList::new();
        let mut future = LinkedList::new();
        // Fill with zeros initially
        for _ in 0..size {
            past.push_back(0);
            future.push_back(0);
        }

        TimeWindow {
            size: size,
            // We initialize the current sample to minus the size of the future list
            // so when the future list is fully populated we will be at zero
            current_sample: -(size as i64),
            sum: 0,
            past: past,
            present: 0,
            future: future,
            dev_total: 0.0,
        }
    }

    // Total size of the window including the past, future and present
    fn total_size(&self) -> usize {
        self.size * 2 + 1
    }

    fn avg(&self) -> f64 {
        (self.sum as f64) / (self.total_size() as f64)
    }

    fn std_dev(&self) -> f64 {
        f64::from(self.dev_total) / (self.total_size() as f64)
    }

    fn current_significant(&self) -> bool {
        self.significant(self.present)
    }

    fn significant(&self, s: Sample) -> bool {
        // Pretty arbitrary
        let diff = (f64::from(s) - self.avg()).abs();
        diff > self.std_dev() * 4.0
    }

    // Add a sample to the window, maybe pushing something
    // out the other end
    fn step_forwards(&mut self, s: Sample) {

        self.current_sample += 1;

        // Push into the most futuristic part of the future
        self.future.push_back(s);

        // Move a oldist future sample into the present
        // Future should always be nonempty
        let popped_future = self.future.pop_front().unwrap();

        // Push the current onto the past
        // then replace present
        self.past.push_back(self.present);
        self.present = popped_future;

        // Pop off oldest sample, Past will always be non-empty
        let popped_back = self.past.pop_front().unwrap();

        // Update the running sum of the samples
        let new_sum = self.sum + i64::from(s) - i64::from(popped_back);

        // Calculate deviation of old sample from old mean and new
        // sample from new mean then update total
        let old_dev = ((self.sum as f64) / (self.total_size() as f64) -
                           f64::from(popped_back)).abs();
        let dev = ((new_sum as f64) / (self.total_size() as f64) -
            f64::from(s)).abs();
        self.dev_total = self.dev_total - old_dev + dev;

        self.sum = new_sum;
    }
}

#[allow(dead_code)]
fn test() {
    let mut tw = TimeWindow::new(2);
    tw.step_forwards(1);
    tw.step_forwards(2);
    println!("{:?}", &tw);
    tw.step_forwards(3);
    tw.step_forwards(4);
    println!("{:?}", &tw);
    tw.step_forwards(5);
    tw.step_forwards(6);
    println!("{:?}", &tw);
    tw.step_forwards(7);
    tw.step_forwards(8);
    println!("{:?}", &tw);
    tw.step_forwards(1);
    tw.step_forwards(2);
    println!("{:?}", &tw);
    println!("Is 8 sig? {}", tw.significant(8));
}
