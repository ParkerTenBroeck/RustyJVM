use super::{ClassBuilderError, FromClassFileIter};

#[derive(Debug)]
pub enum ConstantPoolEntry {
    Empty,
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
        reference_kind: ReferenceKind,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

impl ReferenceKind {
    pub fn from_u8(val: u8) -> Option<Self> {
        Some(match val {
            1 => Self::GetField,
            2 => Self::GetStatic,
            3 => Self::PutField,
            4 => Self::PutStatic,
            5 => Self::InvokeVirtual,
            6 => Self::InvokeStatic,
            7 => Self::InvokeSpecial,
            8 => Self::NewInvokeSpecial,
            9 => Self::InvokeInterface,
            _ => return None,
        })
    }
}

impl ConstantPoolEntry {
    pub fn get_utf8(&self) -> Option<&str> {
        match self {
            Self::Utf8(str) => Some(str),
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
            let two = matches!(
                item,
                ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_)
            );

            vec.push(item);
            if two {
                vec.push(Self::Empty);
                let _ = range.next();
            }
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
                        #[allow(clippy::unusual_byte_groupings)]
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
                reference_kind: ReferenceKind::from_u8(iter.next_u8()?)
                    .map_or(Err(ClassBuilderError::InvalidReferenceKind), Ok)?,
                reference_index: iter.next_u16()?,
            }),
            16 => Ok(ConstantPoolEntry::MethodType {
                descriptor_index: iter.next_u16()?,
            }),
            18 => Ok(ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_attr_index: iter.next_u16()?,
                name_and_type_index: iter.next_u16()?,
            }),
            c => Err(ClassBuilderError::InvalidConstantType(c)),
        }
    }
}

#[derive(Debug)]
pub struct ConstantPool {
    pub constant_pool: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn take(&mut self) -> Self {
        let mut tmp = Vec::new();
        core::mem::swap(&mut tmp, &mut self.constant_pool);
        Self { constant_pool: tmp }
    }

    pub fn get_const_utd8(&self, index: u16) -> Option<&str> {
        if let Some(i) = self.get_constant(index) {
            i.get_utf8()
        } else {
            None
        }
    }

    pub fn get_constant(&self, index: u16) -> Option<&ConstantPoolEntry> {
        self.constant_pool.get(index as usize - 1)
    }

    pub fn get_class_name(&self, index: u16) -> Option<&str> {
        if let Some(i) = self.get_constant(index) {
            if let ConstantPoolEntry::Class { name_index } = i {
                self.get_const_utd8(*name_index)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_class_name_invalid(&self, index: u16) -> &str {
        if let Some(i) = self.get_constant(index) {
            if let ConstantPoolEntry::Class { name_index } = i {
                self.get_const_utd8_or_invalid(*name_index)
            } else {
                "##CONSTANT_NOT_CLASS##"
            }
        } else {
            "##INVALID_INDEX##"
        }
    }

    pub fn get_const_utd8_or_invalid(&self, index: u16) -> &str {
        if let Some(i) = self.get_constant(index) {
            i.get_utf8().unwrap_or("##CONSTANT_NOT_UTF8##")
        } else {
            "##INVALID_INDEX##"
        }
    }

    pub fn new(constant_pool: Vec<ConstantPoolEntry>) -> ConstantPool {
        Self { constant_pool }
    }
}
