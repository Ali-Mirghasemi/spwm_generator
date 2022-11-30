use std::io::{BufWriter, Result, Write};

use spwm_generator::SPWM;

use super::Format;



#[derive(Default)]
pub struct CFile;

impl Format for CFile {
    fn write(&self, name: &str, spwm: &SPWM, width: usize, _sep: &str, buf: &mut BufWriter<std::fs::File>) -> Result<()> {
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

        writeln!(buf, "#include <stdint.h>\n\n")?;
        writeln!(buf, "#define  {}_{}_LEN    {}\n",
            name,
            spwm.sin_freq(),
            table.len(),
        )?;
        writeln!(buf, "const {} {}_{}[{}_{}_LEN] = {{", 
            ty,
            name,
            spwm.sin_freq(), 
            name,
            spwm.sin_freq(),
        )?;

        for row in table.chunks(width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "{:width$}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "}};")?;

        Ok(())
    }
}

#[derive(Default)]
pub struct CHexFile;

impl Format for CHexFile {
    fn write(&self, name: &str, spwm: &SPWM, width: usize, _sep: &str, buf: &mut BufWriter<std::fs::File>) -> Result<()> {
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

        writeln!(buf, "#include <stdint.h>\n\n")?;
        writeln!(buf, "#define  {}_{}_LEN    {}\n",
            name,
            spwm.sin_freq(),
            table.len(),
        )?;
        writeln!(buf, "const {} {}_{}[{}_{}_LEN] = {{", 
            ty,
            name,
            spwm.sin_freq(),
            name,
            spwm.sin_freq(),
        )?;

        for row in table.chunks(width) {
            write!(buf, "    ")?;
            for val in row {
                write!(buf, "0x{:0width$X}, ", val, width=pad_width)?;
            }
            writeln!(buf, "")?;
        }
        
        writeln!(buf, "}};")?;

        Ok(())
    }
}

