
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::{access_flags::ClassAccessFlags, constant_pool};
use crate::attribute::{Attribute, parse_attributes};
use crate::big_endian::ParseBigEndian;
use crate::constant_pool::{Constant, ConstantPoolValidationError};
use crate::field::{Field, FieldParseError, parse_fields};
use crate::method::{Method, MethodParseError, parse_methods};

#[derive(Debug, PartialEq, Eq)]
pub struct JavaVersion {
    pub minor: u16,
    pub major: u16,
}

impl JavaVersion {
    pub fn parse(bytes: [u8; 4]) -> Self {
        let minor_b = [bytes[0], bytes[1]];
        let major_b = [bytes[2], bytes[3]];
        Self {
            minor: minor_b.parse_big_endian(),
            major: major_b.parse_big_endian(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Class {
    /// Java version that this class was compiled for
    pub java_version: JavaVersion,
    /// Contains a pool of constants, like class information, strings, etc...
    pub constant_pool: Vec<Constant>,
    /// Class access flags
    pub access_flags: ClassAccessFlags,
    /// Points to a class in the constant pool that contains the class info for this class
    pub this_class: u16,
    /// Points to a class in the constant pool that contains the class info for the super class
    pub super_class: u16,
    /// The names of the interfaces this class implements
    pub interfaces: Vec<String>,
    /// Fields of this class
    pub fields: Vec<Field>,
    /// Method of this class
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}
pub(crate) fn io_err<T>(res: Result<T, std::io::Error>) -> Result<T, ParseClassError> {
    res.map_err(ParseClassError::IoError)
}
pub(crate) fn read_n_dyn(f: &mut File, n: usize) -> Result<Vec<u8>, ParseClassError> {
    let mut b = vec![0; n];
    io_err(f.read_exact(&mut b))?;
    Ok(b)
}
pub(crate) fn read_n<const N: usize>(f: &mut File) -> Result<[u8; N], ParseClassError> {
    let mut b = [0u8; N];
    io_err(f.read_exact(&mut b))?;
    Ok(b)
}
pub(crate) fn read_u8(f: &mut File) -> Result<u8, ParseClassError> {
    let mut b = [0u8; 1];
    io_err(f.read_exact(&mut b))?;
    Ok(b[0])
}
pub(crate) fn read_u16(f: &mut File) -> Result<u16, ParseClassError> {
    let mut b = [0u8; 2];
    io_err(f.read_exact(&mut b))?;
    Ok(b.parse_big_endian())
}
pub(crate) fn read_u32(f: &mut File) -> Result<u32, ParseClassError> {
    let mut b = [0u8; 4];
    io_err(f.read_exact(&mut b))?;
    Ok(b.parse_big_endian())
}
impl Class {
    pub const MAGIC: u32 = 0xcafebabe;
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, ParseClassError> {

        let path: PathBuf = path.into();
        let mut file = io_err(File::open(path))?;
        if read_u32(&mut file)? != Self::MAGIC {
            return Err(ParseClassError::InvalidMagicNumber);
        }
        let version_bytes = read_n(&mut file)?;
        let java_version = JavaVersion::parse(version_bytes);
        let constant_pool_len: u16 = read_u16(&mut file)? - 1;
        let mut constant_pool = vec![];
        constant_pool.reserve(constant_pool_len as usize);
        for _ in 0..constant_pool_len {
            constant_pool.push(constant_pool::parse_constant(&mut file)?);
        }
        constant_pool::validate_constant_pool(&constant_pool)?;
        let access_flags = ClassAccessFlags::from_bits(read_u16(&mut file)?).unwrap();
        let this_class = read_u16(&mut file)?;
        let super_class = read_u16(&mut file)?;
        let interfaces = io_err(get_interfaces(&mut file, &constant_pool))?;
        let fields = parse_fields(&mut file)?;
        let methods = parse_methods(&mut file)?;
        let attributes = parse_attributes(&mut file)?;
        Ok(Self {
            java_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

fn get_interfaces(f: &mut File, constant_pool: &[Constant]) -> Result<Vec<String>, std::io::Error> {
    fn read_u16(f: &mut File) -> Result<u16, std::io::Error> {
        let mut b = [0u8; 2];
        f.read_exact(&mut b)?;
        Ok(b.parse_big_endian())
    }
    let len = read_u16(f)?;
    let mut interfaces = vec![];
    interfaces.reserve(len as usize);
    for _ in 0..len {
        let class_index = read_u16(f)?;
        let class_name = match &constant_pool[class_index as usize] {
            Constant::Class {
                name_index
            } => match &constant_pool[*name_index as usize] {
                Constant::UTF8String(class_name) => class_name.clone(),
                _ => unreachable!()
            },
            _ => unreachable!()
        };
        interfaces.push(class_name);
    }
    Ok(interfaces)
}

#[derive(Error, Debug)]
pub enum ParseClassError {
    #[error("couldn't read the class file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("expected magic number to be 0xcafebabe")]
    InvalidMagicNumber,
    #[error("invalid utf8 string on constant pool: {0}")]
    InvalidUTF8Constant(#[from] FromUtf8Error),
    #[error("invalid method handle reference kind")]
    InvalidMethodHandleReferenceKind,
    #[error("invalid constant pool: {0}")]
    ConstantPoolValidationError(#[from] ConstantPoolValidationError),
    #[error("failed to parse field: {0}")]
    FieldParseError(#[from] FieldParseError),
    #[error("failed to parse method: {0}")]
    MethodParseError(#[from] MethodParseError)
}
