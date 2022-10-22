use super::{ClassBuilderError, FromClassFileIter};

#[derive(Debug)]
pub enum ConstantPoolEntry {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl ConstantPoolEntry {
    pub fn get_utf8(&self) -> Option<&str> {
        match self {
            Self::Utf8(str) => Some(&str),
            _ => None,
        }
    }
}

impl FromClassFileIter for ConstantPoolEntry {
    fn from_arr(iter: &mut super::ClassFileIter) -> Result<Vec<Self>, ClassBuilderError> {
        let num = iter.next_u16()? - 1;
        let mut vec = Vec::new();
        let mut range = 0..num;
        while let Some(_) = range.next() {
            let item = FromClassFileIter::from_iter(iter)?;
            if matches!(
                item,
                ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_)
            ) {
                let _ = range.next();
            }
            vec.push(item);
        }
        Ok(vec)
    }

    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, ClassBuilderError> {
        match iter.next_u8()? {
            7 => Ok(ConstantPoolEntry::Class {
                name_index: iter.next_u16()?,
            }),
            9 => Ok(ConstantPoolEntry::Fieldref {
                class_index: iter.next_u16()?,
                name_and_type_index: iter.next_u16()?,
            }),
            10 => Ok(ConstantPoolEntry::Methodref {
                class_index: iter.next_u16()?,
                name_and_type_index: iter.next_u16()?,
            }),
            11 => Ok(ConstantPoolEntry::InterfaceMethodref {
                class_index: iter.next_u16()?,
                name_and_type_index: iter.next_u16()?,
            }),
            8 => Ok(ConstantPoolEntry::String {
                string_index: iter.next_u16()?,
            }),
            3 => Ok(ConstantPoolEntry::Integer(iter.next_u32()? as i32)),
            4 => Ok(ConstantPoolEntry::Float(f32::from_bits(iter.next_u32()?))),
            5 => Ok(ConstantPoolEntry::Long(
                (iter.next_u32()? as i64) << 32 | iter.next_u32()? as i64,
            )),
            6 => Ok(ConstantPoolEntry::Double(f64::from_bits(
                (iter.next_u32()? as u64) << 32 | iter.next_u32()? as u64,
            ))),
            12 => Ok(ConstantPoolEntry::NameAndType {
                name_index: iter.next_u16()?,
                descriptor_index: iter.next_u16()?,
            }),
            1 => {
                let len = iter.next_u16()?;
                let mut string = String::with_capacity(len as usize);
                let mut i = 0;
                while i < len {
                    let x = iter.next_u8()?;
                    match x {
                        b'\x01'..=b'\x7F' => {
                            i += 1;
                            string.push(x as char);
                        }
                        0b110_00000..=0b110_11111 => {
                            i += 2;
                            let y = iter.next_u8()?;
                            let c = ((x as u32 & 0x1f) << 6) + (y as u32 & 0x3f);
                            let c = char::from_u32(c)
                                .map_or(Err(ClassBuilderError::StringError), Ok)?;
                            string.push(c);
                        }
                        0b1110_0000..=0b1110_1111 => {
                            i += 2;
                            let y = iter.next_u8()?;
                            let z = iter.next_u8()?;
                            let c = ((x as u32 & 0xf) << 12)
                                + ((y as u32 & 0x3f) << 6)
                                + (z as u32 & 0x3f);
                            let c = char::from_u32(c)
                                .map_or(Err(ClassBuilderError::StringError), Ok)?;
                            string.push(c);
                        }
                        _ => return Err(ClassBuilderError::StringError),
                    }
                }
                Ok(ConstantPoolEntry::Utf8(string))
            }
            15 => Ok(ConstantPoolEntry::MethodHandle {
                reference_kind: iter.next_u8()?,
                reference_index: iter.next_u16()?,
            }),
            16 => Ok(ConstantPoolEntry::MethodType {
                descriptor_index: iter.next_u16()?,
            }),
            18 => Ok(ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_attr_index: iter.next_u16()?,
                name_and_type_index: iter.next_u16()?,
            }),
            c => return Err(ClassBuilderError::InvalidConstantType(c)),
        }
    }
}
