use std::{io::{Result, Write, Seek}, fs::File};

use spwm_generator::SPWM;

use super::{Format, UserSection, FormatArgs};



#[derive(Default)]
pub struct CFile;

impl Format for CFile {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> Result<()> {
        let table = spwm.lookup_table();

        let (ty, pad_width) = if spwm.pwm_top() >= 65536 {
            ("uint32_t", 6)
        }
        else if spwm.pwm_top() >= 256 {
            ("uint16_t", 5)
        }
        else {
            ("uint8_t", 3)
        };

        let sections = UserSection::read_user_reign(buf)?;
        buf.rewind()?;
        buf.set_len(0)?;

        writeln!(buf, "#include <stdint.h>\n\n")?;
        sections.write(0, buf)?;
        writeln!(buf, "#define  {}_{}HZ_CARRIER_FREQ    {}\n",
            args.name,
            spwm.sin_freq(),
            spwm.carrier_freq(),
        )?;
        writeln!(buf, "#define  {}_{}HZ_LEN             {}\n",
            args.name,
            spwm.sin_freq(),
            table.len(),
        )?;
        sections.write(1, buf)?;
        writeln!(buf, "const {} {}_{}HZ[{}_{}HZ_LEN] = {{", 
            ty,
            args.name,
            spwm.sin_freq(), 
            args.name,
            spwm.sin_freq(),
        )?;

        for row in table.chunks(args.width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "{:width$}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }

        writeln!(buf, "}};\n\n")?;
        sections.write(2, buf)?;
        if let Some(pad) = args.inverter {
            let table = spwm.table_not(&table, pad);
            
            writeln!(buf, "const {} {}_{}HZ_NOT[{}_{}HZ_LEN] = {{", 
                ty,
                args.name,
                spwm.sin_freq(),
                args.name,
                spwm.sin_freq(),
            )?;

            for row in table.chunks(args.width) {
                write!(buf, "    ")?;
                for val in row {
                    write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
                }
                writeln!(buf, "")?;
            }

            writeln!(buf, "}};\n\n")?;
        }
        
        sections.write(3, buf)?;
        sections.write_remains(4, buf)?;
        writeln!(buf)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct CHexFile;

impl Format for CHexFile {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> Result<()> {
        let table = spwm.lookup_table();

        let (ty, pad_width) = if spwm.pwm_top() >= 65536 {
            ("uint32_t", 8)
        }
        else if spwm.pwm_top() >= 256 {
            ("uint16_t", 4)
        }
        else {
            ("uint8_t", 2)
        };

        let sections = UserSection::read_user_reign(buf)?;
        buf.rewind()?;
        buf.set_len(0)?;


        writeln!(buf, "#include <stdint.h>\n\n")?;
        sections.write(0, buf)?;
        writeln!(buf, "#define  {}_{}HZ_CARRIER_FREQ    {}\n",
            args.name,
            spwm.sin_freq(),
            spwm.carrier_freq(),
        )?;
        writeln!(buf, "#define  {}_{}HZ_LEN             {}\n",
            args.name,
            spwm.sin_freq(),
            table.len(),
        )?;
        sections.write(1, buf)?;
        writeln!(buf, "const {} {}_{}HZ[{}_{}HZ_LEN] = {{", 
            ty,
            args.name,
            spwm.sin_freq(),
            args.name,
            spwm.sin_freq(),
        )?;

        for row in table.chunks(args.width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }

        writeln!(buf, "}};\n\n")?;
        sections.write(2, buf)?;

        if let Some(pad) = args.inverter {
            let table = spwm.table_not(&table, pad);
            
            writeln!(buf, "const {} {}_{}HZ_NOT[{}_{}HZ_LEN] = {{", 
                ty,
                args.name,
                spwm.sin_freq(),
                args.name,
                spwm.sin_freq(),
            )?;

            for row in table.chunks(args.width) {
                write!(buf, "    ")?;
                for val in row {
                    write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
                }
                writeln!(buf, "")?;
            }

            writeln!(buf, "}};\n\n")?;
        }

        sections.write(3, buf)?;
        sections.write_remains(4, buf)?;
        writeln!(buf)?;

        Ok(())
    }
}

