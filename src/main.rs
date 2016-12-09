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

impl Stream {
    fn new(frame_size: u32, frames_per_sec: u32) -> Self {
        Stream {
            frame_size: frame_size,
            frames_per_sec: frames_per_sec,
        }
    }

    fn sine_wave(&self, amp: f32) -> FrameBuffer {
        use std::f32::consts::PI;

        let mut fb = FrameBuffer {
            frame_size: self.frame_size,
            frames_per_sec: self.frames_per_sec,
            frame: Vec::new(),
        };

        let sample_rate = self.frame_size * self.frames_per_sec;

        for t in (0 .. sample_rate).map(|x| x as f32 / sample_rate as f32) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            fb.frame.push(amp * sample);
        }

        fb
    }
}


fn main() {
    use std::i16;
    let output = Stream::new(44100, 1).sine_wave(1.0);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in output.frame {
        let amplitude = i16::MAX as f32;
        writer.write_sample((t * amplitude) as i16).unwrap();
    }
}
