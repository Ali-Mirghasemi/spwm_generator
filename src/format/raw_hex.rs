use std::{io::{BufWriter, Result, Write, Seek}, fs::File};

use spwm_generator::SPWM;

use super::{Format, UserSection, FormatArgs};



#[derive(Default)]
pub struct RawHex;

impl Format for RawHex {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> Result<()> {
        let table = spwm.lookup_table();

        let sections = UserSection::read_user_reign(buf)?;
        buf.rewind()?;
        buf.set_len(0)?;

        sections.write(0, buf)?;
        for row in table.chunks(args.width) {
            for val in row {
                write!(buf, "{:04X}{}", val, args.separator)?;
            }
            writeln!(buf, "")?;
        }
        sections.write(1, buf)?;
        sections.write_remains(2, buf)?;
        
        Ok(())
    }
}

