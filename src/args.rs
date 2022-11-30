use std::path::PathBuf;

use clap::{Parser, ValueEnum};



#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short = 'f', long = "sin_freq", help = "sin wave frequency")]
    pub sin_freq:           f64,
    #[clap(short = 'n', long = "num_of_samples", help = "number of samples")]
    pub num_of_samples:     usize,
    #[clap(short = 't', long = "pwm_top", default_value = "255", help = "pwm top value")]
    pub pwm_top:            usize,
    #[clap(short = 'p', long = "padding", default_value = "0", help = "padding for pwm min and max value")]
    pub padding:            usize,
    #[clap(short = 'o', long = "out", help = "output file path")]
    pub output:             PathBuf,
    #[clap(short = 'm', long = "format", default_value = "raw", help = "format")]
    pub format:             Format,
    #[clap(short = 'w', long = "row_width", default_value = "16", help = "number of samples in row")]
    pub row_width:          usize,
    #[clap(short = 's', long = "separator", default_value = ", ", help = "separator character")]
    pub separator:          String, 
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
