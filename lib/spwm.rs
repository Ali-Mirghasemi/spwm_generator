use crate::{PWM, SinWave};


/// Hold SPWM parameters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SPWM {
    pwm:            PWM,
    carrier_freq:   f64,
}


impl SPWM {
    /// create new instance
    pub fn new(sin_freq: f64, carrier_freq: f64, pwm_top: usize, padding: usize) -> Self{
        Self { 
            pwm: PWM::new(
                SinWave::new(sin_freq, 1.0), 
                pwm_top, padding
            ), 
            carrier_freq,
        }
    }

    /// return carrier frequency
    pub fn carrier_freq(&self) -> f64 {
        self.carrier_freq
    }

    /// set carrier frequency
    pub fn set_carrier_freq(&mut self, carrier_freq: f64) {
        self.carrier_freq = carrier_freq;
    }

    pub fn sin_freq(&self) -> f64 {
        self.pwm.freq()
    }

    pub fn pwm_top(&self) -> usize {
        self.pwm.pwm_top()
    }

    /// generate lookup table
    pub fn lookup_table(&self) -> Vec<usize> {
        self.pwm.duty_cycles_range(0.0, 1.0, 1.0 / self.carrier_freq)
    }
}

