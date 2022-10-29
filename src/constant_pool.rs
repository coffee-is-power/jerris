use std::fs::File;

use num_traits::FromPrimitive;
use thiserror::*;

use crate::class;
use crate::class::ParseClassError;

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Eq)]
pub enum MethodReferenceKind {
    GetField = 1,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Class {
        /// This index references to a utf8 string in the constant pool
        name_index: u16
    },
    Field {
        /// Index of the class this fields belongs to in the constant pool
        class_index: u16,
        /// Index of the name and type in the constant pool
        name_and_type_index: u16,
    },
    Method {
        /// Index of the class this method belongs to in the constant pool
        class_index: u16,
        /// Index of the name and type in the constant pool
        name_and_type_index: u16,
    },
    InterfaceMethod {
        /// Index of the class this interface method belongs to in the constant pool
        class_index: u16,
        /// Index of the name and type in the constant pool
        name_and_type_index: u16,
    },
    String {
        /// Index of the utf8 string on the constant pool
        string_index: u16
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        /// Points to a utf8 item in the constant pool which is the name of this element
        name_index: u16,
        /// The value of the descriptor_index item must be a valid index into the constant_pool table.
        /// The constant_pool entry at that index must be a CONSTANT_Utf8_info structure representing a valid field descriptor or method descriptor.
        descriptor_index: u16,
    },
    UTF8String(String),
    MethodHandle {
        reference_kind: MethodReferenceKind,
        /// ## From the java specification:
        ///
        /// * The value of the reference_index item must be a valid index into the constant_pool table:
        ///
        /// * If the value of the reference_kind item is 1 (REF_getField), 2 (REF_getStatic), 3 (REF_putField), or 4 (REF_putStatic), then the constant_pool entry at that index must be a CONSTANT_Fieldref_info (§4.4.2) structure representing a field for which a method handle is to be created.
        ///
        /// * If the value of the reference_kind item is 5 (REF_invokeVirtual), 6 (REF_invokeStatic), 7 (REF_invokeSpecial), or 8 (REF_newInvokeSpecial), then the constant_pool entry at that index must be a CONSTANT_Methodref_info structure (§4.4.2) representing a class's method or constructor (§2.9) for which a method handle is to be created.
        ///
        /// * If the value of the reference_kind item is 9 (REF_invokeInterface), then the constant_pool entry at that index must be a CONSTANT_InterfaceMethodref_info (§4.4.2) structure representing an interface's method for which a method handle is to be created.
        ///
        /// * If the value of the reference_kind item is 5 (REF_invokeVirtual), 6 (REF_invokeStatic), 7 (REF_invokeSpecial), or 9 (REF_invokeInterface), the name of the method represented by a CONSTANT_Methodref_info structure must not be <init> or <clinit>.
        ///
        /// * If the value is 8 (REF_newInvokeSpecial), the name of the method represented by a CONSTANT_Methodref_info structure must be <init>.
        reference_index: u16,
    },
    MethodType {
        /// Points to a string representing the signature of this method
        descriptor_index: u16,
    },
    InvokeDynamic {
        /// The value of the bootstrap_method_attr_index item must be a valid index into the bootstrap_methods array of the bootstrap method table (§4.7.21) of this class file.
        bootstrap_method_attr_index: u16,
        /// Points to a name and type in the constant pool
        name_and_type_index: u16,
    },
}

#[derive(Error, Debug)]
pub enum ConstantPoolValidationError {
    #[error("expected name_index of class to point to a string")]
    ClassWithInvalidNameIndex,
    #[error("method has invalid class index")]
    MethodWithInvalidClassIndex,
    #[error("method has invalid name and type index")]
    MethodWithInvalidNameAndTypeIndex,
    #[error("field has invalid class index")]
    FieldWithInvalidClassIndex,
    #[error("field has invalid name and type index")]
    FieldWithInvalidNameAndTypeIndex,
    #[error("interface method has invalid class index")]
    InterfaceMethodWithInvalidClassIndex,
    #[error("interface method has invalid name and type index")]
    InterfaceMethodWithInvalidNameAndTypeIndex,
    #[error("string object has invalid utf8 string index")]
    StringWithInvalidUTF8Index,
    #[error("name and type has invalid name index")]
    NameAndTypeWithInvalidNameIndex,
    #[error("name and type has invalid descriptor index")]
    NameAndTypeWithInvalidDescriptorIndex,
    #[error("invoke dynamic has invalid name and type index")]
    InvokeDynamicWithInvalidNameAndType,
    #[error("invoke dynamic has invalid bootstrap method index")]
    InvokeDynamicWithInvalidBootstrapMethodIndex,
    #[error("method type has invalid descriptor index")]
    MethodTypeWithInvalidDescriptorIndex,
    #[error("invalid method handle")]
    InvalidMethodHandle,
}

fn validate_constant(constant: &Constant, pool: &[Constant]) -> Result<(), ConstantPoolValidationError> {
    match constant {
        Constant::Class { name_index } => {
            let name_constant = &pool[*name_index as usize];
            if matches!(name_constant, Constant::UTF8String(_)) {
                validate_constant(name_constant, pool)?;
                Ok(())
            } else {
                Err(ConstantPoolValidationError::ClassWithInvalidNameIndex)
            }
        }
        Constant::Method { class_index, name_and_type_index } => {
            let class_constant = &pool[*class_index as usize];
            if matches!(class_constant, Constant::Class {..}) {
                validate_constant(class_constant, pool)?;
                let nat_constant = &pool[*name_and_type_index as usize];
                if matches!(nat_constant, Constant::NameAndType {..}) {
                    validate_constant(nat_constant, pool)?;
                    Ok(())
                } else {
                    Err(ConstantPoolValidationError::MethodWithInvalidNameAndTypeIndex)
                }
            } else {
                Err(ConstantPoolValidationError::MethodWithInvalidClassIndex)
            }
        }
        Constant::Field { class_index, name_and_type_index } => {
            let class_constant = &pool[*class_index as usize];
            if matches!(class_constant, Constant::Class {..}) {
                validate_constant(class_constant, pool)?;
                let nat_constant = &pool[*name_and_type_index as usize];
                if matches!(nat_constant, Constant::NameAndType {..}) {
                    validate_constant(nat_constant, pool)?;
                    Ok(())
                } else {
                    Err(ConstantPoolValidationError::FieldWithInvalidNameAndTypeIndex)
                }
            } else {
                Err(ConstantPoolValidationError::FieldWithInvalidClassIndex)
            }
        }
        Constant::InterfaceMethod { class_index, name_and_type_index } => {
            let class_constant = &pool[*class_index as usize];
            if matches!(class_constant, Constant::Class {..}) {
                validate_constant(class_constant, pool)?;
                let nat_constant = &pool[*name_and_type_index as usize];
                if matches!(nat_constant, Constant::NameAndType {..}) {
                    validate_constant(nat_constant, pool)?;
                    Ok(())
                } else {
                    Err(ConstantPoolValidationError::InterfaceMethodWithInvalidNameAndTypeIndex)
                }
            } else {
                Err(ConstantPoolValidationError::InterfaceMethodWithInvalidClassIndex)
            }
        }
        // Just assume they're good, nothing to check here
        Constant::Integer(_) | Constant::Long(_) | Constant::Float(_) | Constant::Double(_) => Ok(()),
        Constant::UTF8String(_) => Ok(()),
        Constant::String { string_index } => {
            let string_constant = &pool[*string_index as usize];
            if matches!(string_constant, Constant::UTF8String(_)) {
                validate_constant(string_constant, pool)?;
                Ok(())
            } else {
                Err(ConstantPoolValidationError::StringWithInvalidUTF8Index)
            }
        }
        Constant::NameAndType { name_index, descriptor_index } => {
            let name_constant = &pool[*name_index as usize];
            if matches!(name_constant, Constant::UTF8String(_)) {
                validate_constant(name_constant, pool)?;
                let descriptor_constant = &pool[*descriptor_index as usize];
                if matches!(descriptor_constant, Constant::UTF8String(_)) {
                    validate_constant(descriptor_constant, pool)?;
                    Ok(())
                } else {
                    Err(ConstantPoolValidationError::NameAndTypeWithInvalidDescriptorIndex)
                }
            } else {
                Err(ConstantPoolValidationError::NameAndTypeWithInvalidNameIndex)
            }
        }
        Constant::InvokeDynamic { name_and_type_index, .. } => {
            let nat_constant = &pool[*name_and_type_index as usize];
            if matches!(nat_constant, Constant::NameAndType{..}) {
                validate_constant(nat_constant, pool)?;
                eprintln!("FIXME!: Implement bootstrap method attr index check");
                Ok(())
            } else {
                Err(ConstantPoolValidationError::InvokeDynamicWithInvalidNameAndType)
            }
        }
        Constant::MethodType { descriptor_index } => {
            let descriptor_constant = &pool[*descriptor_index as usize];
            if matches!(descriptor_constant, Constant::UTF8String(_)) {
                validate_constant(descriptor_constant, pool)?;
                Ok(())
            } else {
                Err(ConstantPoolValidationError::MethodTypeWithInvalidDescriptorIndex)
            }
        }
        Constant::MethodHandle { reference_index, reference_kind } => {
            use MethodReferenceKind::*;
            match reference_kind {
                GetField | GetStatic | PutField | PutStatic => {
                    let field_constant = &pool[*reference_index as usize];
                    if matches!(field_constant, Constant::Field{..}) {
                        validate_constant(field_constant, pool)?;
                        Ok(())
                    } else {
                        Err(ConstantPoolValidationError::InvalidMethodHandle)
                    }
                }
                InvokeSpecial | InvokeVirtual | InvokeStatic => {
                    let method_constant = &pool[*reference_index as usize];
                    if matches!(method_constant, Constant::Method{..}) {
                        validate_constant(method_constant, pool)?;
                        Ok(())
                    } else {
                        Err(ConstantPoolValidationError::InvalidMethodHandle)
                    }
                }
                InvokeInterface => {
                    let interface_method_constant = &pool[*reference_index as usize];
                    if matches!(interface_method_constant, Constant::InterfaceMethod{..}) {
                        validate_constant(interface_method_constant, pool)?;
                        Ok(())
                    } else {
                        Err(ConstantPoolValidationError::InvalidMethodHandle)
                    }
                }
                NewInvokeSpecial => {
                    let diamond_init_method_constant = &pool[*reference_index as usize];
                    if matches!(diamond_init_method_constant, Constant::Method{..}) {
                        validate_constant(diamond_init_method_constant, pool)?;
                        let name_and_type = match diamond_init_method_constant {
                            Constant::Method { name_and_type_index, .. } => &pool[*name_and_type_index as usize],
                            _ => unreachable!()
                        };
                        let name = match name_and_type {
                            Constant::NameAndType { name_index, .. } => &pool[*name_index as usize],
                            _ => unreachable!()
                        };
                        let name = match name {
                            Constant::UTF8String(name) => name,
                            _ => unreachable!()
                        };
                        if &**name == "<init>" {
                            Ok(())
                        } else {
                            Err(ConstantPoolValidationError::InvalidMethodHandle)
                        }
                    } else {
                        Err(ConstantPoolValidationError::InvalidMethodHandle)
                    }
                }
            }
        }
    }
}

pub fn validate_constant_pool(constant_pool: &[Constant]) -> Result<(), ParseClassError> {
    for constant in constant_pool {
        validate_constant(constant, constant_pool).map_err(ParseClassError::ConstantPoolValidationError)?;
    }
    Ok(())
}

pub fn parse_constant(f: &mut File) -> Result<Constant, ParseClassError> {
    let tag = class::read_u8(f)?;
    match tag {
        // UTF8
        1 => {
            let len = class::read_u16(f)?;
            let bytes = class::read_n_dyn(f, len as usize)?;
            let string = String::from_utf8(bytes).map_err(ParseClassError::InvalidUTF8Constant)?;
            Ok(Constant::UTF8String(string))
        }
        // Method handle
        15 => {
            let reference_kind: MethodReferenceKind = match FromPrimitive::from_u8(class::read_u8(f)?) {
                Some(rk) => Ok(rk),
                None => Err(ParseClassError::InvalidMethodHandleReferenceKind)
            }?;
            let reference_index = class::read_u16(f)? - 1;
            Ok(Constant::MethodHandle {
                reference_kind,
                reference_index,
            })
        }
        // Method Type
        16 => {
            Ok(Constant::MethodType {
                descriptor_index: class::read_u16(f)? - 1
            })
        }
        // Invoke Dynamic
        18 => {
            let bootstrap_method_attr_index = class::read_u16(f)? - 1;
            let name_and_type_index = class::read_u16(f)? - 1;
            Ok(Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            })
        }
        // Name And Type
        12 => {
            let name_index = class::read_u16(f)? - 1;
            let descriptor_index = class::read_u16(f)? - 1;
            Ok(Constant::NameAndType {
                descriptor_index,
                name_index,
            })
        }
        // Integer
        3 => {
            Ok(Constant::Integer(class::read_u32(f)? as i32))
        }
        // Float
        4 => {
            Ok(Constant::Float(f32::from_bits(class::read_u32(f)?)))
        }
        // Long
        5 => {
            let high = class::read_u32(f)? as u64;
            let low = class::read_u32(f)? as u64;
            Ok(Constant::Long(((high << 32) | low) as i64))
        }
        // Double
        6 => {
            let high = class::read_u32(f)? as u64;
            let low = class::read_u32(f)? as u64;
            Ok(Constant::Double(f64::from_bits((high << 32) | low)))
        }
        // Class
        7 => {
            Ok(Constant::Class {
                name_index: class::read_u16(f)? - 1
            })
        }
        // String
        8 => {
            Ok(Constant::String {
                string_index: class::read_u16(f)? - 1
            })
        }
        // Field
        9 => {
            let class_index = class::read_u16(f)? - 1;
            let name_and_type_index = class::read_u16(f)? - 1;
            Ok(Constant::Field { class_index, name_and_type_index })
        }
        // Method
        10 => {
            let class_index = class::read_u16(f)? - 1;
            let name_and_type_index = class::read_u16(f)? - 1;
            Ok(Constant::Method { class_index, name_and_type_index })
        }
        // Interface Method
        11 => {
            let class_index = class::read_u16(f)? - 1;
            let name_and_type_index = class::read_u16(f)? - 1;
            Ok(Constant::InterfaceMethod { class_index, name_and_type_index })
        }
        _ => todo!()
    }
}
