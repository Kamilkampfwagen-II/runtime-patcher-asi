use std::{error::Error, fmt};

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
pub enum PatchError {
    ByteMismatch(u32, u8, u8),
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PatchError::ByteMismatch(offset, expected, found) => write!(f, "Byte {:#x} at target offset {:#x} does not match the original byte {:#x} in the patch.", found, offset, expected)
        }
    }
}
impl Error for PatchError {}

pub mod i337 {
    use std::error::Error;

    use super::{Patch, PatchSet};

    pub fn parse(content: &str) -> Result<PatchSet, Box<dyn Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let first_line = *lines
            .first()
            .ok_or("Not a valid 1337, module name was not found.")?;
        let module = first_line[1..].to_owned();

        let mut patches: Vec<Patch> = vec![];

        for line in &lines[1..] {
            let split_colon: Vec<&str> = line.split(':').collect();
            let split_arrow: Vec<&str> = split_colon
                .get(1)
                .ok_or("Not a valid 1337, no entry was found.")?
                .split("->")
                .collect();

            let offset = u32::from_str_radix(
                split_colon
                    .first()
                    .ok_or("Not a valid 1337, failed to parse the offset.")?,
                16,
            )?;
            let org = u8::from_str_radix(
                split_arrow
                    .first()
                    .ok_or("Not a valid 1337, failed to parse the original byte.")?,
                16,
            )?;
            let new = u8::from_str_radix(
                split_arrow
                    .get(1)
                    .ok_or("Not a valid 1337, failed to parse the new byte.")?,
                16,
            )?;

            patches.push(Patch { offset, org, new })
        }

        Ok(PatchSet {
            module,
            set: patches,
        })
    }
}
