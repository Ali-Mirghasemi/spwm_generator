use std::io::{BufWriter, Result, Write};

use spwm_generator::SPWM;

use super::Format;




#[derive(Default)]
pub struct Raw;

impl Format for Raw {
    fn write(&self, _name: &str, spwm: &SPWM, width: usize, sep: &str, buf: &mut BufWriter<std::fs::File>) -> Result<()> {
        let table = spwm.lookup_table();

        for row in table.chunks(width) {
            for val in row {
                write!(buf, "{:3}{}", val, sep)?;
            }
            writeln!(buf, "")?;
        }
        
        Ok(())
    }
}

