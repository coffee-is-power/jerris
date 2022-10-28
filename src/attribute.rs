use std::fs::File;

use crate::class::{ParseClassError, read_u16, read_u32, read_u8};

#[derive(Debug, PartialEq, Eq)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}

pub(crate) fn parse_attribute(f: &mut File) -> Result<Attribute, ParseClassError> {
    let attribute_name_index = read_u16(f)?;
    let attr_len = read_u32(f)?;
    let mut info = vec![];
    info.reserve(attr_len as usize);
    for _ in 0..attr_len {
        info.push(read_u8(f)?);
    }
    Ok(Attribute {
        info,
        attribute_name_index,
    })
}

pub(crate) fn parse_attributes(f: &mut File) -> Result<Vec<Attribute>, ParseClassError> {
    let len = read_u16(f)?;
    let mut attributes = vec![];
    for _ in 0..len {
        let attr = parse_attribute(f)?;
        attributes.push(attr);
    }
    Ok(attributes)
}