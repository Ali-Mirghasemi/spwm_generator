use std::io::ErrorKind;

use crate::{PWM, SinWave, DutyCycle};

use rayon::prelude::*;

/// Hold SPWM parameters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SPWM {
    pwm:                PWM,
    num_of_samples:     usize,
    step:               f64,
}


impl SPWM {
    pub fn convert(duration: f64, num_of_samples: Option<usize>, step: Option<f64>, carrier_freq: Option<f64>) -> Result<(usize, f64), std::io::Error> {
        if let Some(freq) = carrier_freq {
            let s = 1.0 / freq;
            let n = (duration / s) as usize;
            Ok((n, s))
        }
        else {
            let n = if let Some(x) = num_of_samples {
                x
            }
            else {
                if let Some(x) = step {
                    (duration / x) as usize
                }
                else {
                    return Err(std::io::Error::new(ErrorKind::Other, "time_step parameter need when num_of_samples not provide"));
                }
            };
    
            let s = if let Some(x) = step {
                x
            }
            else {
                if let Some(x) = num_of_samples {
                    duration / x as f64
                }
                else {
                    return Err(std::io::Error::new(ErrorKind::Other, "num_of_samples parameter need when time_step not provide"));
                }
            };

            Ok((n, s))
        }
    }

    /// create new instance
    pub fn new(sin_freq: f64, num_of_samples: usize, step: f64, pwm_top: DutyCycle, padding: usize) -> Self{
        Self { 
            pwm: PWM::new(
                SinWave::new(sin_freq, 1.0), 
                pwm_top, padding as DutyCycle
            ), 
            num_of_samples: if num_of_samples == 0 {
                1
            }
            else {
                num_of_samples
            },
            step,
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

    pub fn pwm_top(&self) -> DutyCycle {
        self.pwm.pwm_top()
    }

    pub fn carrier_freq(&self) -> f64 {
        1.0 / self.step
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn set_step(&mut self, step: f64) {
        self.step = step;
    }
 
    /// generate lookup table
    pub fn lookup_table(&self) -> Vec<DutyCycle> {
        self.pwm.duty_cycles(0.0, self.num_of_samples, self.step)
    }

    pub fn table_not(&self, table: &Vec<DutyCycle>, pad: f64) -> Vec<DutyCycle> {
        let p = (pad * self.pwm_top() as f64 / self.step) as DutyCycle;
        table.par_iter().map(|x| {
            let mut v = x - p;

            if v > self.pwm_top() {
                v = self.pwm_top();
            }
            else if v < 0 {
                v = 0;
            }

            v
        }).collect()
    }

}

