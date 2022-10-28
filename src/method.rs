use std::fs::File;

use thiserror::Error;

use crate::access_flags::MethodAccessFlags;
use crate::attribute::{Attribute, parse_attributes};
use crate::class::{ParseClassError, read_u16};

#[derive(Debug, PartialEq, Eq)]
pub struct Method {
    /// Method's access flags
    pub access_flags: MethodAccessFlags,
    /// Method name
    pub name_index: u16,
    /// Method signature
    ///
    /// See: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.3.3
    pub descriptor_index: u16,
    /// Method attributes
    pub attributes: Vec<Attribute>,
}

#[derive(Error, Debug)]
pub enum MethodParseError {
    #[error("method has invalid access flags")]
    InvalidAccessFlags,
}

pub(crate) fn parse_method(f: &mut File) -> Result<Method, ParseClassError> {
    let access_flags = MethodAccessFlags::from_bits(read_u16(f)?);
    let access_flags = match access_flags {
        Some(mf) => Ok(mf),
        None => Err(ParseClassError::MethodParseError(MethodParseError::InvalidAccessFlags))
    }?;
    let name_index = read_u16(f)?;
    let descriptor_index = read_u16(f)?;
    let attributes = parse_attributes(f)?;
    Ok(Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

pub(crate) fn parse_methods(f: &mut File) -> Result<Vec<Method>, ParseClassError> {
    let len = read_u16(f)?;
    let mut result = vec![];
    result.reserve(len as usize);
    for _ in 0..len {
        result.push(parse_method(f)?);
    }
    Ok(result)
}