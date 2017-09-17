use std::collections::{LinkedList, HashMap};
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::sync::mpsc::Sender;
use std::u16::MAX as U16MAX;
use std::u32::MAX as U32MAX;

use common::{AudioType, AudioPacket};
use hound::{Sample, WavReader};


pub fn run_audio<T : Read>(
    mut reader : WavReader<T>,
    tx : Sender<AudioPacket>,
    sample_time : f64,
    start_time : SystemTime) {

    let spec = reader.spec();
    let channels = spec.channels;
    let sample_rate = spec.sample_rate;

    // For now we window over a half a second (Completely arbitrary)
    // This is a quater of a second forwards and backwards in time
    let window_size = sample_rate as f64 * sample_time;
    let mut window = TimeWindow::new(window_size as usize);

    // Whether we are currently above the significant threshold and triggering
    let mut triggered = false;

    for (i, sample) in reader.samples::<i32>().enumerate() {

        // Skip over all wav channels but the first
        if i % channels as usize != 0 {
            continue;
        }

        match sample {
            // Not absing the signal works better odly
            Ok(x) => parse_sample(x, &tx, &start_time, sample_rate, &mut window, &mut triggered),
            Err(e) => println!("ERROR {:?}", e),
        };
    }
}

// Ugly function
fn parse_sample(
    x : i32,
    tx : &Sender<AudioPacket>,
    start_time : &SystemTime,
    sample_rate : u32,
    window : &mut TimeWindow<i32>,
    triggered : &mut bool) {

    // Add the new sample to the window
    window.step_forwards(x);

    // Check if the new sample is significant
    let sig = window.current_significant();

    // If we are switiching to a new state
    if sig != *triggered {
    
        *triggered = sig;

        // Calculate the difference between the time in the song the triggering
        // happened and the current time in the song's playback
        let trigger_time = window.current_sample as f64 / sample_rate as f64;
        let trigger_time_dur = Duration::from_millis((trigger_time * 1000.0) as u64);

        match start_time.elapsed()
                        .ok()
                        .and_then(|current_songtime| {
            trigger_time_dur.checked_sub(current_songtime)
        }) {
            Some(time_diff) => {

                // Sleep until the point in the song where we were triggered
                sleep(time_diff);
            },
            None => {
                // Errors can occur at several points
                // SystemTime can get out of sync randomly
                // We can get behind the song
                // We can be triggered before the song starts
                //
                // I think there are some problems in the first 3 + backfill
                // seconds as well
            }
        }

        // Send update to the graphics
        let intensity = x as f64 / (U16MAX as f64);
        let mut audio_map: HashMap<AudioType, f64> = HashMap::new();
        audio_map.insert(AudioType::Intensity, intensity);
        let update = AudioPacket {
            time : trigger_time_dur,
            audio : audio_map};
        try_send_update(&tx, update);

    }
}


fn try_send_update(tx : &Sender<AudioPacket>, update : AudioPacket) {

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
struct TimeWindow<S : Sample> {
    size : usize,

    current_sample : i64,

    // The sum of all the samples the window
    // can view
    sum      : i64,

    // The sum of the difference of all the samples
    // from the mean
    dev_total : f64,

    past     : LinkedList<S>,
    present  : S,
    future   : LinkedList<S>,
}

impl TimeWindow<i32> {

    fn new (size : usize) -> Self {
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
            size : size,
            // We initialize the current sample to minus the size of the future list
            // so when the future list is fully populated we will be at zero
            current_sample : - (size as i64),
            sum : 0,
            past : past,
            present : 0,
            future : future,
            dev_total : 0.0,
        }
    }

    // Total size of the window including the past, future and present
    fn total_size(&self) -> usize {
        self.size * 2 + 1
    }

    fn avg(&self) -> f64 {
        self.sum as f64 / self.total_size() as f64
    }

    fn std_dev(&self) -> f64 {
        self.dev_total as f64 / self.total_size() as f64
    }

    fn current_significant(&self) -> bool {
        self.significant(self.present)
    }

    fn significant(&self, s : i32) -> bool {
        // Pretty arbitrary
        let diff =  (s as f64 - self.avg()).abs();
        diff > self.std_dev() * 4.0
    }

    // Add a sample to the window, maybe pushing something
    // out the other end
    fn step_forwards(&mut self, s : i32) {

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
        let new_sum = self.sum + (s as i64) - (popped_back as i64);

        // Calculate deviation of old sample from old mean and new
        // sample from new mean then update total
        let old_dev = ((self.sum as f64) / (self.total_size() as f64) - (popped_back as f64)).abs();
        let dev = ((new_sum as f64) / (self.total_size() as f64) - (s as f64)).abs();
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
