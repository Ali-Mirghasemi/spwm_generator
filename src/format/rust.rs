use std::io::{BufWriter, Result, Write};

use spwm_generator::SPWM;

use super::Format;

#[derive(Default)]
pub struct RustFile;

impl Format for RustFile {
    fn write(&self, name: &str, spwm: &SPWM, width: usize, _sep: &str, buf: &mut BufWriter<std::fs::File>) -> Result<()> {
        let table = spwm.lookup_table();

        let (ty, pad_width) = if spwm.pwm_top() >= 65536 {
            ("u32", 6)
        }
        else if spwm.pwm_top() >= 256 {
            ("u16", 5)
        }
        else {
            ("u8", 3)
        };

        writeln!(buf, "const {}_{}[{}; {}] = [", 
            name,
            spwm.sin_freq(), 
            ty,
            table.len()
        )?;

        for row in table.chunks(width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "{:width$}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "];")?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RustHexFile;

impl Format for RustHexFile {
    fn write(&self, name: &str, spwm: &SPWM, width: usize, _sep: &str, buf: &mut BufWriter<std::fs::File>) -> Result<()> {
        let table = spwm.lookup_table();

        let (ty, pad_width) = if spwm.pwm_top() >= 65536 {
            ("u32", 8)
        }
        else if spwm.pwm_top() >= 256 {
            ("u16", 4)
        }
        else {
            ("u8", 2)
        };

        writeln!(buf, "const {}_{}[{}; {}] = [", 
            name,
            spwm.sin_freq(), 
            ty,
            table.len()
        )?;

        for row in table.chunks(width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "];")?;

        Ok(())
    }
}


