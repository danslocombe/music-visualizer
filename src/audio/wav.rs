extern crate hound;

use audio::{AudioData, Song};
use audio::wav::hound::{Error, Sample, WavReader};
use std::io::Read;
use std::time::Duration;

pub struct WavSong<T: Read> {
    reader: WavReader<T>,
    current_sample: usize,
    channels: usize,
    sample_rate: u32,
}

impl<T: Read> WavSong<T> {
    pub fn new(t: T) -> Result<Self, Error> {
        let reader = WavReader::new(t)?;
        let spec = reader.spec();
        Ok(WavSong {
            reader: reader,
            current_sample: 0,
            channels: spec.channels as usize,
            sample_rate: spec.sample_rate,
        })
    }
}

impl<T: Read> Song for WavSong<T> {
    // This is actually only correct for 16 bit encoded wavs, and will break
    // for others
    fn sample_max_value(&self) -> u32 {
        1 << self.reader.spec().bits_per_sample
    }
}

impl<T: Read> Iterator for WavSong<T> {
    type Item = AudioData;
    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..self.channels - 1 {
            let _: i32 = self.reader.samples().next().unwrap().unwrap();
        }
        let x = self.reader.samples().next().map(|s| {
            let t = (self.current_sample as f64) / f64::from(self.sample_rate);
            let dur = Duration::from_millis((t * 1000.0) as u64);
            AudioData {
                time: dur,
                sample: s.unwrap(),
            }
        });
        self.current_sample += 1;
        x
    }
}
