extern crate hound;

#[derive(Debug)]
struct Stream {
    frames_per_sec: u32,
}

#[derive(Debug)]
struct AudioFrame {
    frames_per_sec: u32,
    frame: f32,
}

impl AudioFrame {
    fn new(frames_per_sec: u32, frame: f32) -> AudioFrame {
        AudioFrame {
            frames_per_sec: frames_per_sec,
            frame: frame,
        }
    }
}

#[derive(Debug)]
struct SilenceGenerator {
    frames_per_sec: u32,
}

impl Iterator for SilenceGenerator {
    type Item = AudioFrame;

    fn next(&mut self) -> Option<Self::Item> {
        Some(AudioFrame {
            frames_per_sec: self.frames_per_sec,
            frame: 0.0
        })
    }
}

#[derive(Debug)]
struct SineGenerator {
    frames_per_sec: u32,
    phase: u32,
    freq: f32,
}

trait AudioIteratorMethods: Iterator<Item=AudioFrame>+Sized {
    fn amp(self, amplitude: f32)->AmpIterator<Self>;
}

impl Iterator for SineGenerator {
    type Item = AudioFrame;

    fn next(&mut self) -> Option<Self::Item> {
        use std::f32::consts::PI;

        let t = self.phase as f32 / self.frames_per_sec as f32;

        let frame = (t * self.freq * 2.0 * PI).sin();

        self.phase = (self.phase + 1) % self.frames_per_sec;

        Some(AudioFrame {
            frames_per_sec: self.frames_per_sec,
            frame: frame,
        })
    }
}

impl AudioIteratorMethods for SineGenerator {
    fn amp(self, amplitude: f32)->AmpIterator<Self> {
        AmpIterator {
            frames_per_sec: self.frames_per_sec,
            amplitude: amplitude,
            iter: self
        }
    }
}

struct AmpIterator<T: Iterator<Item=AudioFrame>+Sized> {
    frames_per_sec: u32,
    amplitude: f32,
    iter: T,
}

impl<T: Iterator<Item=AudioFrame>+Sized> Iterator for AmpIterator<T> {
    type Item = AudioFrame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => Some(AudioFrame::new(x.frames_per_sec, x.frame * self.amplitude)),
            None => None
        } 
    }
}

impl Stream {
    fn new(frames_per_sec: u32) -> Self {
        Stream {
            frames_per_sec: frames_per_sec,
        }
    }

    fn sine(&self, freq: f32) -> SineGenerator {
        SineGenerator {
            frames_per_sec: self.frames_per_sec,
            phase: 0,
            freq: freq,
        }
    }

    fn silence(&self) -> SilenceGenerator {
        SilenceGenerator {
            frames_per_sec: self.frames_per_sec
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

    for buffer in output.sine(440.0).amp(0.3).take(SAMPLE_RATE as usize) {
        let t = buffer.frame;
        let amplitude = i16::MAX as f32;
        writer.write_sample((t * amplitude) as i16).unwrap();
    }
    for buffer in output.silence().take(SAMPLE_RATE as usize) {
        let t = buffer.frame;
        let amplitude = i16::MAX as f32;
        writer.write_sample((t * amplitude) as i16).unwrap();
    }
    for buffer in output.sine(660.0).amp(0.3).take(SAMPLE_RATE as usize) {
        let t = buffer.frame;
        let amplitude = i16::MAX as f32;
        writer.write_sample((t * amplitude) as i16).unwrap();
    }
}
