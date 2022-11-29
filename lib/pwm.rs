use crate::SinWave;
use rayon::prelude::*;


/// This struct hold SPWM parameters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PWM {
    wave:           SinWave,
    pwm_top:        usize,
    padding:        usize,
}

impl PWM {
    /// create new instance
    pub fn new(mut wave: SinWave, pwm_top: usize, padding: usize) -> Self {
        wave.set_offset(0.0);
        Self {
            wave,
            pwm_top,
            padding,
        }
    }

    /// return wave frequency
    pub fn freq(&self) -> f64 {
        self.wave.freq()
    }

    pub fn pwm_top(&self) -> usize {
        self.pwm_top
    }

    /// This function return a signal value at given time
    /// 
    /// ```rust
    /// use spwm_generator::*;
    /// 
    /// let wave = SinWave::new(1.0, 1.0, 0.0);
    /// let pwm = PWM::new(wave, 255, 0);
    /// 
    /// let val = pwm.duty_cycle(0.0);
    /// assert!(val, 127);
    /// 
    /// let val = pwm.duty_cycle(0.5);
    /// assert!(val, 255);
    /// ```
    /// 
    pub fn duty_cycle(&self, time: f64) -> usize {
        let mut val = ((self.pwm_top as f64 / 2.0) + ((self.wave.sample(time) * (self.pwm_top as f64 / 2.0)) / self.wave.amplitude())) as usize;
        if val + self.padding > self.pwm_top {
            val = self.pwm_top - self.padding;
        }
        else if val - self.padding > self.pwm_top {
            val = self.padding;
        }
        val
    }

    /// This function generate a series of samples by start and end time
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0, 0.0);
    /// let pwm = PWM::new(wave, 255, 0);
    ///
    /// let duty_cycles = pwm.duty_cycles_range(0.0, 1.0, 0.125);
    /// println!("{:?}", duty_cycles);
    /// ```
    pub fn duty_cycles_range(&self, start_time: f64, end_time: f64, step: f64) -> Vec<usize> {
        let num_samples = ((end_time - start_time) / step) as usize;

        self.duty_cycles(start_time, num_samples, step)
    }

    /// This function generate a series of samples
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0, 0.0);
    /// let pwm = PWM::new(wave, 255, 0);
    ///
    /// let duty_cycles = pwm.duty_cycles_fixed(0.0, 1.0, 8);
    /// println!("{:?}", duty_cycles);
    /// ```
    pub fn duty_cycles_fixed(&self, start_time: f64, end_time: f64, num_samples: usize) -> Vec<usize> {
        let step = (end_time - start_time) / num_samples as f64;
        
        self.duty_cycles(start_time, num_samples, step)
    }

    /// This function generate a series of samples by number of samples
    /// 
    /// ```rust
    /// let wave = SinWave::new(1.0, 1.0, 0.0);
    /// let pwm = PWM::new(wave, 255, 0);
    ///
    /// let duty_cycles = pwm.duty_cycles(0.0, 8, 0.125);
    /// println!("{:?}", duty_cycles);
    /// ```
    pub fn duty_cycles(&self, start_time: f64, num_samples: usize, step: f64) -> Vec<usize> {
        (0..num_samples).into_par_iter()
            .map(|x| {
                self.duty_cycle(start_time + x as f64 * step)
            })
            .collect()
    }

}


