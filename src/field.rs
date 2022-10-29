use std::fs::File;

use thiserror::Error;

use crate::access_flags::FieldAccessFlags;
use crate::attribute::{Attribute, parse_attributes};
use crate::class::{ParseClassError, read_u16};

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    /// Access flags of this field
    pub access_flags: FieldAccessFlags,
    /// Index of the name of this field in the constant pool
    pub name_index: u16,
    /// The type index on the constant pool of this field
    pub descriptor_index: u16,
    /// TODO: attribute field docs
    pub attributes: Vec<Attribute>,
}

#[derive(Error, Debug)]
pub enum FieldParseError {
    #[error("field has invalid access flags")]
    InvalidAccessFlags,
}

fn parse_field(f: &mut File) -> Result<Field, ParseClassError> {
    let access_flags = FieldAccessFlags::from_bits(read_u16(f)?);
    let access_flags = match access_flags {
        Some(af) => Ok(af),
        None => Err(ParseClassError::FieldParseError(FieldParseError::InvalidAccessFlags))
    }?;
    let name_index = read_u16(f)?;
    let descriptor_index = read_u16(f)?;
    let attributes = parse_attributes(f)?;
    Ok(Field {
        name_index,
        descriptor_index,
        access_flags,
        attributes,
    })
}

pub(crate) fn parse_fields(f: &mut File) -> Result<Vec<Field>, ParseClassError> {
    let len = read_u16(f)?;
    let mut result = vec![];
    result.reserve(len as usize);
    for _ in 0..len {
        result.push(parse_field(f)?);
    }
    Ok(result)
}