use std::u16::MAX as U16MAX;
use std::io::Read;
use std::time::Duration;
use hound::{Sample, WavReader, Error};
use audio::{Song, AudioData};
use rodio::{Decoder, Source};
use rodio;
use std::io::BufReader;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct WavSong<T : Read> {
    reader : WavReader<T>,
    current_sample : usize,
    channels : usize,
    sample_rate : u32,
    name : PathBuf,
}

impl<T : Read> WavSong<T> {
    pub fn new(t : T, path: &Path) -> Result<Self, Error> {
        let reader = WavReader::new(t)?;
        let spec = reader.spec();
        let name = path.to_path_buf();
        Ok(WavSong {
            reader : reader,
            current_sample : 0,
            channels : spec.channels as usize,
            sample_rate : spec.sample_rate,
            name : name,
        })
    }
}

impl<T : Read> Song for WavSong<T> {
    // This is actually only correct for 16 bit encoded wavs, and will break
    // for others
    fn sample_max_value(&self) -> u32 {
        1 << self.reader.spec().bits_per_sample
    }

    fn play(&self) {
        let endpoint = rodio::get_default_endpoint().unwrap();
        let file = File::open(self.name.clone()).unwrap();
        let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();

        rodio::play_raw(&endpoint, decoder.convert_samples());
    }
}

impl<T : Read> Iterator for WavSong<T> {
    type Item = AudioData;
    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0 .. self.channels - 1 {
            let _ : i32 = self.reader.samples().next().unwrap().unwrap();
        }
        let x = self.reader.samples().next().map(|s| {
            let t = self.current_sample as f64 / self.sample_rate as f64;
            let dur = Duration::from_millis((t * 1000.0) as u64);
            AudioData {
                time : dur,
                sample : s.unwrap(),
            }
        });
        self.current_sample += 1;
        x
    }
}
