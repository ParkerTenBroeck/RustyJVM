use self::{attribute::*, constant::*, field::*, interface::*, method::*};

pub mod attribute;
pub mod constant;
pub mod field;
pub mod interface;
pub mod method;

enum MinorVersion {}

enum MajorVersion {}

mycelium_bitfield::bitfield! {
    /// Bitfield types can have doc comments.
    #[derive(Eq, PartialEq)] // ...and attributes
    pub struct AccessFlags<u16> {
        pub const PUBLIC: bool;

        const _RESERVED = 3;

        pub const FINAL: bool;
        pub const SUPER: bool;


        const _RESERVED2 = 3;

        pub const INTERFACE: bool;
        pub const ABSTRACT: bool;

        const _RESERVED3 = 1;

        pub const SYNTHETIC: bool;
        pub const ANNOTATION: bool;
        pub const ENUM: bool;
    }
}

pub struct Class {
    minor_version: u16,
    major_version: u16,

    //const_pool_count: u16,
    constant_pool: Vec<ConstantPoolEntry>,

    access_flags: AccessFlags,

    this_class: u16,
    super_class: u16,

    //interfaces_count: u16,
    interfaces: Vec<InterfaceEntry>,

    // field_count: u16,
    field_info: Vec<FieldEntry>,

    // methods_count: u16,
    method_info: Vec<MethodEntry>,

    // attributes_count: u16,
    attribute_info: Vec<AttributeEntry>,
}

pub enum ClassBuilderError {
    InvalidMagic,
    ReachedEndOfFile,
}

impl Class {
    pub fn new(data: &[u8]) -> Result<Self, ClassBuilderError> {
        let mut iter = ClassFileIter::new(data);

        let val = iter.next_u32()?;
        if val != 0xCAFEBABE {
            return Err(ClassBuilderError::InvalidMagic);
        }

        Ok(Self {
            minor_version: iter.next_u16()?,
            major_version: iter.next_u16()?,
            constant_pool: FromClassFileIter::from_arr(&mut iter)?,
            access_flags: AccessFlags::from_bits(iter.next_u16()?),
            this_class: iter.next_u16()?,
            super_class: iter.next_u16()?,
            interfaces: FromClassFileIter::from_arr(&mut iter)?,
            field_info: FromClassFileIter::from_arr(&mut iter)?,
            method_info: FromClassFileIter::from_arr(&mut iter)?,
            attribute_info: FromClassFileIter::from_arr(&mut iter)?,
        })
    }
}

trait FromClassFileIter: Sized {
    fn from(iter: &mut ClassFileIter) -> Result<Self, ClassBuilderError>;
    fn from_arr(iter: &mut ClassFileIter) -> Result<Vec<Self>, ClassBuilderError> {
        let num = iter.next_u16()?;
        let mut vec = Vec::new();
        for _ in 0..num {
            vec.push(Self::from(iter)?);
        }
        Ok(vec)
    }
}

pub struct ClassFileIter<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> ClassFileIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            slice: data,
            index: 0,
        }
    }
    pub fn next_u8(&mut self) -> Result<u8, ClassBuilderError> {
        self.index += 1;
        self.slice
            .get(self.index - 1)
            .map_or(Err(ClassBuilderError::ReachedEndOfFile), |s| Ok(*s))
    }

    pub fn next_u16(&mut self) -> Result<u16, ClassBuilderError> {
        Ok(((self.next_u8()? as u16) << 8) | ((self.next_u8()? as u16) << 0))
    }

    pub fn next_u32(&mut self) -> Result<u32, ClassBuilderError> {
        Ok(((self.next_u8()? as u32) << 24)
            | ((self.next_u8()? as u32) << 16)
            | ((self.next_u8()? as u32) << 8)
            | ((self.next_u8()? as u32) << 0))
    }
}
