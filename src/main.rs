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
    frame_size: u32,
    frames_per_sec: u32,
}

#[derive(Debug)]
struct FrameBuffer {
    frame_size: u32,
    frames_per_sec: u32,
    frame: Vec<f32>,
}

#[derive(Debug)]
struct DelayIterator {
    frame_size: u32,
    frames_per_sec: u32,
    phase: u32,
    delay: u32, // delay in # samples
}


#[derive(Debug)]
struct SineIterator {
    frame_size: u32,
    frames_per_sec: u32,
    phase: u32,
    amp: f32,
    freq: f32,
}

impl Iterator for SineIterator {
    type Item = FrameBuffer;

    fn next(&mut self) -> Option<Self::Item> {
        use std::f32::consts::PI;

        let mut fb = FrameBuffer {
            frame_size: self.frame_size,
            frames_per_sec: self.frames_per_sec,
            frame: Vec::new(),
        };

        let sample_rate = self.frame_size * self.frames_per_sec;

        for t in (self.phase .. (self.phase + self.frame_size))
                                .map(|x| x as f32 / sample_rate as f32) {
            let sample = (t * self.freq * 2.0 * PI).sin();
            fb.frame.push(self.amp * sample);
        }

        self.phase += self.frame_size;
        self.phase = self.phase % sample_rate;

        Some(fb)
    }
}

impl Stream {
    fn new(frame_size: u32, frames_per_sec: u32) -> Self {
        Stream {
            frame_size: frame_size,
            frames_per_sec: frames_per_sec,
        }
    }

    fn sine(&self, freq: f32, amp: f32) -> SineIterator {
        SineIterator {
            frame_size: self.frame_size,
            frames_per_sec: self.frames_per_sec,
            phase: 0,
            amp: amp,
            freq: freq,
        }
    }
}


fn main() {
    use std::i16;

    let output = Stream::new(100, 441);
    
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    let mut num_samples = 0;

    for buffer in output.sine(880.0, 1.0) {
        for t in buffer.frame {
            let amplitude = i16::MAX as f32;
            writer.write_sample((t * amplitude) as i16).unwrap();
            num_samples += 1;
            if num_samples == 44100 {
                return;
            }            
        }
    }
}
