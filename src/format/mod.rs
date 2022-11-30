use std::{io::BufWriter, fs::File};

use spwm_generator::SPWM;



mod raw;
mod raw_hex;
mod c;
mod rust;

pub use raw::*;
pub use raw_hex::*;
pub use c::*;
pub use rust::*;

pub trait Format {
    fn write(&self, name: &str, spwm: &SPWM, width: usize, sep: &str, buf: &mut BufWriter<File>) -> std::io::Result<()>;
}

