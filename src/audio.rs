
use sdl2::audio::{ AudioCallback, AudioSpecDesired };

struct SineWave
{
    tone_frequency: f32,
    sample_rate:    u16,
    amplitude:      u16,
}

impl SineWave
{
    fn samples(&self) -> Vec<i16>
    {
        let increment   = self.tone_frequency / self.sample_rate as f32;

        // Num the samples needed to get a full cycle to 2pi.
        let num_samples = (1.0 / increment).ceil() as isize;

        let generator = | step: f32 |
            {
                let value = self.amplitude as f32 * (step * 2.0 * std::f32::consts::PI).sin();
                return value as i16;
            };

        let samples = (0..num_samples)
                        .map(|x| x as f32 * increment)
                        .map(generator)
                        .collect::<Vec<i16>>();

        return samples;
    }
}

struct SpeakersCallback
{
    samples: Vec<i16>,
    current_sample: usize,
}

impl SpeakersCallback
{
    fn new(tone_frequency: f32, sample_rate: u16, amplitude: u16) -> SpeakersCallback
    {
        let wave = SineWave { tone_frequency, sample_rate, amplitude };

        return SpeakersCallback { samples: wave.samples(), current_sample: 0 };
    }
}

impl AudioCallback for SpeakersCallback
{
    type Channel = i16;

    fn callback(&mut self, out: &mut [Self::Channel])
    {
        for x in out.iter_mut()
        {
            if self.current_sample == self.samples.len()
            {
                self.current_sample = 0;
            }

            *x = self.samples[self.current_sample];
            self.current_sample += 1;
        }
    }
}

pub struct Speakers
{
    device: sdl2::audio::AudioDevice<SpeakersCallback>,
}

impl Speakers
{
    pub fn new() -> Speakers
    {
        let context         = sdl2::init().unwrap();
        let audio_subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired
        {
            freq:     Some(44100),
            channels: Some(1),
            samples:  None
        };

        let get_callback = |spec: sdl2::audio::AudioSpec|
        {
           SpeakersCallback::new(440.0, spec.freq as u16, 30000)
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, get_callback);

        return Speakers { device: device.unwrap() };
    }

    pub fn start(&self)
    {
        self.device.resume();
    }

    pub fn stop(&self)
    {
        self.device.pause();
    }

    pub fn is_playing(&self) -> bool
    {
        return self.device.status() == sdl2::audio::AudioStatus::Playing;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::time::Duration;
    use crate::helpers::tests::*;

    #[test]
    fn sample_generation()
    {
        let wave = SineWave { tone_frequency: 440.0,
                              sample_rate:    44100,
                              amplitude:      30000
                            };

        let golden_samples = vec![0, 1879, 3751, 5608, 7444, 9250, 11019, 12746, 14422, 16042, 17598,
                              19086, 20498, 21830, 23077, 24232, 25293, 26254, 27111, 27863, 28505,
                              29035, 29450, 29750, 29934, 29999, 29947, 29777, 29490, 29088, 28571,
                              27941, 27202, 26356, 25407, 24358, 23213, 21977, 20654, 19250, 17771,
                              16222, 14609, 12939, 11218, 9453, 7650, 5818, 3963, 2092, 213,
                              -1666, -3539, -5398, -7236, -9046, -10820, -12552, -14234, -15861,
                              -17425, -18921, -20342, -21683, -22940, -24106, -25177, -26150,
                              -27019, -27783, -28437, -28980, -29409, -29722, -29919, -29998,
                              -29959, -29802, -29529, -29139, -28635, -28018, -27292, -26458,
                              -25520, -24482, -23347, -22121, -20808, -19414, -17943, -16401,
                              -14795, -13131, -11416, -9655, -7857, -6028, -4175, -2305, -427];

        let samples = wave.samples();

        assert_eq!( samples, golden_samples);
    }

    #[test]
    fn speakers_playback() -> Result<(), String>
    {
        let mutex = test_lock()?;

        let speakers = Speakers::new();
        assert!(!speakers.is_playing());

        speakers.start();
        assert!(speakers.is_playing());

        speakers.stop();
        assert!(!speakers.is_playing());

        speakers.start();
        std::thread::sleep(Duration::from_millis(1000));
        assert!(speakers.is_playing());

        speakers.stop();
        assert!(!speakers.is_playing());

        Ok(())
    }
}
