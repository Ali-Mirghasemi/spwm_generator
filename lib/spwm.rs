use crate::{PWM, SinWave};


/// Hold SPWM parameters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SPWM {
    pwm:                PWM,
    num_of_samples:     usize,
}


impl SPWM {
    /// create new instance
    pub fn new(sin_freq: f64, num_of_samples: usize, pwm_top: usize, padding: usize) -> Self{
        Self { 
            pwm: PWM::new(
                SinWave::new(sin_freq, 1.0), 
                pwm_top, padding
            ), 
            num_of_samples: num_of_samples,
        }
    }

    /// return carrier frequency
    pub fn num_of_samples(&self) -> usize {
        self.num_of_samples
    }

    /// set carrier frequency
    pub fn set_num_of_samples(&mut self, num_of_samples: usize) {
        self.num_of_samples = num_of_samples;
    }

    pub fn sin_freq(&self) -> f64 {
        self.pwm.freq()
    }

    pub fn pwm_top(&self) -> usize {
        self.pwm.pwm_top()
    }

    /// generate lookup table
    pub fn lookup_table(&self) -> Vec<usize> {
        self.pwm.duty_cycles_fixed(0.0, 1.0, self.num_of_samples)
    }
}

