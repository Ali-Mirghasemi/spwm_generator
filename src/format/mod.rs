use std::{io::{Read, Write}, fs::File, iter::Map, collections::{HashMap, btree_map::Range}};

use spwm_generator::SPWM;



mod raw;
mod raw_hex;
mod c;
mod rust;

pub use raw::*;
pub use raw_hex::*;
pub use c::*;
pub use rust::*;

#[derive(Debug, Clone, Default)]
pub struct UserSection {
    pub sections:           HashMap<i32, String>,
}

const USER_SECTION_BEGIN: &'static str    = "// USER SECTION BEGIN";
const USER_SECTION_END: &'static str      = "// USER SECTION END";

enum SectionState {
    None,
    Section,
}

#[derive(Debug, Clone)]
pub struct FormatArgs {
    pub name:           String,
    pub separator:      String,
    pub width:          usize,
    pub inverter:       Option<f64>,
}

pub trait Format {
    fn write(&self, spwm: &SPWM, buf: &mut File, args: &FormatArgs) -> std::io::Result<()>;
}

impl UserSection {

    pub fn write(&self, index: i32, fs: &mut File) -> std::io::Result<()> {
        writeln!(fs, "{} {}", USER_SECTION_BEGIN, index)?;
        if let Some(txt) = self.sections.get(&index) {
            write!(fs, "{}", txt)?;
        }
        writeln!(fs, "{} {}\n", USER_SECTION_END, index)
    }

    pub fn write_remains(&self, offset: i32, fs: &mut File) -> std::io::Result<()> {
        if (offset as usize) < self.sections.len() {
            for section in self.sections.iter() {
                if *section.0 >= offset {
                    self.write(*section.0, fs)?;
                }
            }
        }
        Ok(())
    }

    pub fn read_user_reign(fs: &mut File) -> std::io::Result<UserSection> {
        let mut buf = String::new();
        fs.read_to_string(&mut buf)?;

        let mut state = SectionState::None;
        let mut user = UserSection::default();

        let mut temp = (-1, String::new());

        for mut line in buf.lines() {
            match state {
                SectionState::None => {
                    line = line.trim();

                    if line.starts_with(USER_SECTION_BEGIN) {
                        temp.0 = line[USER_SECTION_BEGIN.len()..].trim().parse().unwrap();
                        state = SectionState::Section;
                    }
                },
                SectionState::Section => {
                    let temp_line = line.trim();
                    if temp_line.starts_with(USER_SECTION_END) &&
                        temp_line.len() > USER_SECTION_END.len() &&
                        temp.0 == temp_line[USER_SECTION_END.len()..].trim().parse().unwrap() 
                    {
                        state = SectionState::None;
                        user.sections.insert(temp.0, temp.1);
                        temp = (-1, String::new());
                    }
                    else {
                        temp.1.push_str(line);
                        temp.1.push_str("\n");
                    }
                },
            }
        }

        Ok(user)
    }
}



