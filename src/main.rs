extern crate hound;

use std::env;
use std::collections::LinkedList;

use hound::Sample;


fn main() {

    // Load wav from first arg
    let arg = env::args().nth(1).unwrap();
    let mut reader = hound::WavReader::open(&arg).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let sample_rate = spec.sample_rate;

    // For now we window over a quater of a second (Completely arbitrary)
    // This is a quater of a second forwards and backwards in time
    let window_size = sample_rate / 4;
    let mut window = TimeWindow::new(window_size as usize);

    // Whether we are currently above the significant threshold and triggering
    let mut triggered = false;

    for (i, sample) in reader.samples::<i32>().enumerate() {

        // Skip over all channels but the first
        if i % channels as usize != 0 {
            continue;
        }

        match sample {
            Ok(x) => {
                // Add the new sample to the window
                window.step_forwards(x);

                // Check if new sample significant
                let sig = window.current_significant();

                if sig != triggered {
                    triggered = sig;

                    if triggered {
                        let current_time = window.current_sample as f64 / sample_rate as f64;
                        println!("TRIGGERING AT TIME {}", current_time);
                    }
                }
            }
            Err(e) => println!("ERROR {:?}", e),
        };
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
