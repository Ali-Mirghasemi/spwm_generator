use std::{io::{BufWriter}, fs::File};

use args::Args;
use clap::Parser;
use spwm_generator::*;

mod args;
mod format;

use format::*;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let spwm = SPWM::new(
        args.sin_freq, 
        args.num_of_samples, 
        args.pwm_top, 
        args.padding
    );
    
    let fs = File::create(&args.output)?;
    let mut buf = BufWriter::new(fs);
    let writer: &dyn Format = match args.format {
        args::Format::Raw => &Raw,
        args::Format::RawHex => &RawHex,
        args::Format::C => &CFile,
        args::Format::CHex => &CHexFile,
        args::Format::Rust => &RustFile,
        args::Format::RustHex => &RustHexFile,
    };
    writer.write(&args.name, &spwm, args.row_width, &args.separator, &mut buf)
}
