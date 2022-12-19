use std::{io::{BufWriter, Result, Write, Seek}, fs::File};

use spwm_generator::SPWM;

use super::{Format, UserSection, FormatArgs};

#[derive(Default)]
pub struct RustFile;

impl Format for RustFile {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> Result<()> {
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

        let sections = UserSection::read_user_reign(buf)?;
        buf.rewind()?;
        buf.set_len(0)?;

        
        sections.write(0, buf)?;
        writeln!(buf, "const {}_{}HZ[{}; {}] = [", 
            args.name,
            spwm.sin_freq(), 
            ty,
            table.len()
        )?;

        for row in table.chunks(args.width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "{:width$}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "];")?;
        sections.write(1, buf)?;

        if let Some(pad) = args.inverter {
            let table = spwm.table_not(&table, pad);

            writeln!(buf, "const {}_{}HZ_NOT[{}; {}] = [", 
                args.name,
                spwm.sin_freq(), 
                ty,
                table.len()
            )?;

            for row in table.chunks(args.width) {
                write!(buf, "    ")?;
                for val in row {
                    write!(buf, "{:width$}, ", val, width=pad_width)?;
                }
                writeln!(buf, "")?;
            }
            
            writeln!(buf, "];")?;
        }



        sections.write(2, buf)?;
        sections.write_remains(3, buf)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RustHexFile;

impl Format for RustHexFile {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> Result<()> {
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

        let sections = UserSection::read_user_reign(buf)?;
        buf.rewind()?;
        buf.set_len(0)?;

        sections.write(0, buf)?;
        writeln!(buf, "const {}_{}HZ[{}; {}] = [", 
            args.name,
            spwm.sin_freq(), 
            ty,
            table.len()
        )?;

        for row in table.chunks(args.width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "];")?;
        sections.write(1, buf)?;

        if let Some(pad) = args.inverter {
            let table = spwm.table_not(&table, pad);

            writeln!(buf, "const {}_{}HZ_NOT[{}; {}] = [", 
                args.name,
                spwm.sin_freq(), 
                ty,
                table.len()
            )?;

            for row in table.chunks(args.width) {
                write!(buf, "    ")?;
                for val in row {
                    write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
                }
                writeln!(buf, "")?;
            }
            
            writeln!(buf, "];")?;
        }

        sections.write(2, buf)?;
        sections.write_remains(3, buf)?;

        Ok(())
    }
}


