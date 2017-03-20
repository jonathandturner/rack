/*
extern crate rayon;

use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}
*/

extern crate hound;

#[derive(Debug)]
struct Stream {
    frames_per_sec: u32,
}

#[derive(Debug)]
struct FrameBuffer {
    frames_per_sec: u32,
    frame: f32,
}

#[derive(Debug)]
struct SineIterator {
    frames_per_sec: u32,
    phase: u32,
    amp: f32,
    freq: f32,
}

impl Iterator for SineIterator {
    type Item = FrameBuffer;

    fn next(&mut self) -> Option<Self::Item> {
        use std::f32::consts::PI;

        let t = self.phase as f32 / self.frames_per_sec as f32;

        let frame = (t * self.freq * 2.0 * PI).sin() * self.amp;

        self.phase += 1;
        self.phase %= self.frames_per_sec;

        Some(FrameBuffer {
            frames_per_sec: self.frames_per_sec,
            frame: frame,
        })
    }
}

impl Stream {
    fn new(frames_per_sec: u32) -> Self {
        Stream {
            frames_per_sec: frames_per_sec,
        }
    }

    fn sine(&self, freq: f32, amp: f32) -> SineIterator {
        SineIterator {
            frames_per_sec: self.frames_per_sec,
            phase: 0,
            amp: amp,
            freq: freq,
        }
    }
}


fn main() {
    use std::i16;

    const SAMPLE_RATE:u32 = 44100;

    let output = Stream::new(SAMPLE_RATE);
    
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    let mut num_samples = 0;

    for buffer in output.sine(440.0, 1.0) {
        let t = buffer.frame;
        let amplitude = i16::MAX as f32;
        writer.write_sample((t * amplitude) as i16).unwrap();
        num_samples += 1;
        if num_samples == SAMPLE_RATE {
            return;
        }            
    }
}
