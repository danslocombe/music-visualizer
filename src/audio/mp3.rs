extern crate simplemad;
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::iter::FlatMap;
use std::rc::Rc;
use self::simplemad::{Decoder, Frame, MadFixed32};
use audio::AudioData;

pub struct Mp3Song<T : Read> {
    //type Item = AudioData;
    start_time : SystemTime,
    decoder : Decoder<T>,
    current_frame : Option<Frame>,
    current_frame_sample : usize,
    current_time : Duration,
}

impl<T : Read> Mp3Song<T> {
    // Fetch the next mp3 frame from the file and swap out the current frame
    // Returns whether it was successful
    fn update_frame(&mut self) -> bool{
        self.decoder.next()
                    .and_then(|mframe| {
                        mframe.ok().and_then(|frame| {
                self.current_frame = Some(frame);
                self.current_frame_sample = 0;
                Some(())
            })
        }).is_some()

    }

    // Get the current sample from the current frame
    fn get_sample_from_frame(&self) -> Option<MadFixed32> {
        self.current_frame.as_ref().and_then(|frame| {
            if self.current_frame_sample < frame.samples[0].len() {
                Some(frame.samples[0][self.current_frame_sample])
            }
            else {
                None
            }
        })
    }

    fn update(&mut self) {
        let frame = self.current_frame.as_ref().unwrap();
        self.current_frame_sample += 1;
        self.current_time += frame.duration / frame.samples[0].len() as u32;
    }
}

impl<T : Read> Iterator for Mp3Song<T> {
    type Item = AudioData;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.get_sample_from_frame();

        let sample2 = match sample {
            Some(x) => {
                // Sample was fetched ok, iterate the current sample and go
                self.update();
                Some(x)
            }
            None  => {
                // Try and fetch the next frame
                // If that succeeds try and get a sample from it
                if (self.update_frame()) {
                    // If we just got an empty frame we end the stream
                    // but maybe we should try the next one?
                    let x = self.get_sample_from_frame();
                    if (x.is_some()) {
                        self.update();
                    }
                    x
                }
                else {
                    println!("Could not udpate");
                    None
                }
            }
        };

        // If we still have None at this point then we have run out of mp3
        // so we return none
        sample2.map(|s| {
            // Otherwise construct an AudioData packet
            AudioData {
                sample : s.to_i32(),
                time : self.current_time,
            }
        })
    }
}


pub fn run_audio<T : Read>(r : T, start_time : SystemTime) -> Mp3Song<T>{
    let mut decoder = Decoder::decode(r).unwrap();
    // Skip over first few
    let mut mframe = None;
    for frame in &mut decoder {
        match frame {
            Ok(x) => {
                mframe = Some(x);
                break;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    let mut s = Mp3Song {
        decoder: decoder, 
        start_time : start_time,
        current_frame : mframe,
        current_frame_sample : 0,
        current_time : Duration::new(0, 0),
    };
    s

}
