use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use spwm_generator::DutyCycle;



#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short = 'f', long = "sin_freq", help = "sin wave frequency")]
    pub sin_freq:           f64,
    #[clap(short = 'n', long = "num_of_samples", help = "number of samples")]
    pub num_of_samples:     Option<usize>,
    #[clap(short = 'c', long = "carrier_freq", help = "carrier frequency or PWM frequency")]
    pub carrier_freq:       Option<f64>,
    #[clap(short = 's', long = "step", help = "time step in seconds")]
    pub step:               Option<f64>,
    #[clap(short = 't', long = "pwm_top", default_value = "255", help = "pwm top value")]
    pub pwm_top:            DutyCycle,
    #[clap(short = 'p', long = "padding", default_value = "0", help = "padding for pwm min and max value")]
    pub padding:            usize,
    #[clap(short = 'o', long = "out", help = "output file path")]
    pub output:             PathBuf,
    #[clap(short = 'm', long = "format", default_value = "raw", help = "format")]
    pub format:             Format,
    #[clap(short = 'w', long = "row_width", default_value = "16", help = "number of samples in row")]
    pub row_width:          usize,
    #[clap(short = 'S', long = "separator", default_value = ", ", help = "separator character")]
    pub separator:          String, 
    #[clap(short = 'a', long = "name", default_value = "WAVE", help = "name of variable")]
    pub name:               String, 
    #[clap(short = 'C', long = "cycle", help = "lookup table just for one cycle, this parameter override duration parameter")]
    pub one_cycle:          bool,
    #[clap(short = 'd', long = "duration", default_value = "1.0", help = "duration of lookup table")]
    pub duration:           f64,
    #[clap(short = 'P', long = "plot", help = "plot spwm wave")]
    pub plot:               Option<PlotMode>,
    #[clap(short = 'I', long = "inverter", help = "inverter mode")]
    pub inverter:           Option<f64>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Raw,
    RawHex,
    C,
    CHex,
    Rust,
    RustHex,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum PlotMode {
    PWM,
    CenterAligned,
}
