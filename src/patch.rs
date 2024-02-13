use std::fmt;

pub struct Patch {
    pub offset: u32,
    pub org: u8,
    pub new: u8,
}

pub struct PatchSet {
    pub module: String,
    pub set: Vec<Patch>,
}

#[derive(Debug)]
pub enum Error {
    ByteMismatch(u32, u8, u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ByteMismatch(offset, expected, found) => write!(f, "Byte {:#x} at target offset {:#x} does not match the original byte {:#x} in the patch.", found, offset, expected)
        }
    }
}
impl std::error::Error for Error {}

pub mod i337 {
    use regex::Regex;
    use std::num::ParseIntError;

    use super::{Patch, PatchSet};

    #[derive(Debug)]
    pub enum Error {
        MissingModuleName,
        MissingEntry,
        OffsetParseError(ParseIntError),
        OriginalByteParseError(ParseIntError),
        NewByteParseError(ParseIntError),
    }

    pub fn parse(content: &str) -> Result<PatchSet, Error> {
        let lines: Vec<&str> = content.lines().collect();
        let first_line = *lines.first().ok_or(Error::MissingModuleName)?;
        let module = first_line[1..].to_owned();

        let mut patches: Vec<Patch> = vec![];
        let regex = Regex::new(r"^(.*):(.*)->(.*)$").unwrap();

        for line in &lines[1..] {
            if let Some(caps) = regex.captures(line) {
                let offset = u32::from_str_radix(&caps[1], 16).map_err(Error::OffsetParseError)?;
                let org =
                    u8::from_str_radix(&caps[2], 16).map_err(Error::OriginalByteParseError)?;
                let new = u8::from_str_radix(&caps[3], 16).map_err(Error::NewByteParseError)?;

                patches.push(Patch { offset, org, new });
            } else {
                return Err(Error::MissingEntry);
            }
        }

        Ok(PatchSet {
            module,
            set: patches,
        })
    }
}
