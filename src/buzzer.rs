extern crate sdl2;

use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Buzzer {
    device: AudioDevice<SquareWave>,
}

impl Buzzer {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Buzzer {
        let spec = AudioSpecDesired {
            freq: Some(3000),
            channels: Some(1),
            samples: None
        };
        
        let device = audio_subsystem.open_playback(None, &spec, |spec| {
          SquareWave {
              phase_inc: 440.0 / spec.freq as f32,
              phase: 0.0,
              volume: 0.25
          }
        }).unwrap();
        
        Buzzer { device }
    }

    pub fn set(&self, state: bool) {
        if state {
            self.device.resume();
        } else {
            self.device.pause();
        }
    }
}
