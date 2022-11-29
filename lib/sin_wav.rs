use std::f64::consts::PI;

use rayon::prelude::*;

/// Holder for sin wave parameters
/// 
/// ```rust
/// let wave = SinWave::new(50.0f, 1.0f)
/// ```
///
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct SinWave {
    freq:           f64,
    amplitude:      f64,
    offset:         f64,
}


impl SinWave {
    /// create new instance
    pub fn new(freq: f64, amplitude: f64) -> Self {
        Self {
            freq,
            amplitude,
            offset: 0.0,
        }
    }

    /// create new instance with offset
    pub fn new_offset(freq: f64, amplitude: f64, offset: f64) -> Self {
        Self {
            freq,
            amplitude,
            offset,
        }
    }

    /// return wave amplitude
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// set wave amplitude
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude;
    }

    /// return wave frequency
    pub fn freq(&self) -> f64 {
        self.freq
    }

    /// set wave frequency
    pub fn set_freq(&mut self, freq: f64) {
        self.freq = freq;
    }

    /// return wave offset 
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// set wave offset
    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }

    /// This function return a signal value at given time
    /// 
    /// ```rust
    /// use spwm_generator::*;
    /// 
    /// let wave = SinWave::new(1.0, 1.0);
    /// 
    /// let val = wave.sample(0.0);
    /// assert!(val, 0.0);
    /// 
    /// let val = wave.sample(0.5);
    /// assert!(val, 1.0);
    /// ```
    /// 
    pub fn sample(&self, time: f64) -> f64 {
        self.offset + (self.amplitude * (2.0 * PI * self.freq * time).sin())
    }

    /// This function generate a series of samples by start and end time
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0);
    ///
    /// let samples = wave.samples_range(0.0, 1.0, 0.125);
    /// println!("{:?}", samples);
    /// ```
    pub fn samples_range(&self, start_time: f64, end_time: f64, step: f64) -> Vec<f64> {
        let num_samples = ((end_time - start_time) / step) as usize;

        self.samples(start_time, num_samples, step)
    }

    /// This function generate a series of samples
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0);
    ///
    /// let samples = wave.samples_fixed(0.0, 1.0, 0.125);
    /// println!("{:?}", samples);
    /// ```
    pub fn samples_fixed(&self, start_time: f64, end_time: f64, num_samples: usize) -> Vec<f64> {
        let step = (end_time - start_time) / num_samples as f64;
        
        self.samples(start_time, num_samples, step)
    }

    /// This function generate a series of samples by number of samples
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0);
    ///
    /// let samples = wave.samples(0.0, 4, 0.125);
    /// println!("{:?}", samples);
    /// ```
    pub fn samples(&self, start_time: f64, num_samples: usize, step: f64) -> Vec<f64> {
        (0..num_samples).into_par_iter()
            .map(|x| {
                self.sample(start_time + x as f64 * step)
            })
            .collect()
    }

}

impl Default for SinWave {
    fn default() -> Self {
        Self {
            freq: 1.0,
            amplitude: 1.0,
            offset: 0.0,
        }
    }
}

impl From<f64> for SinWave {
    fn from(val: f64) -> Self {
        Self {
            freq: val,
            amplitude: 1.0,
            offset: 0.0,
        }
    }
}

impl From<(f64, f64)> for SinWave {
    fn from(val: (f64, f64)) -> Self {
        Self {
            freq: val.0,
            amplitude: val.1,
            offset: 0.0,
        }
    }
}

impl From<(f64, f64, f64)> for SinWave {
    fn from(val: (f64, f64, f64)) -> Self {
        Self {
            freq: val.0,
            amplitude: val.1,
            offset: val.2,
        }
    }
}

