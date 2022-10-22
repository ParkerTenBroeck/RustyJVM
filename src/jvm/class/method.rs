use super::{FromClassFileIter, attribute::AttributeEntry};


mycelium_bitfield::bitfield! {
    /// Bitfield types can have doc comments.
    #[derive(Eq, PartialEq)] // ...and attributes
    pub struct AccessFlags<u16> {
        pub const PUBLIC: bool;
        pub const PRIVATE: bool;
        pub const PROTECTED: bool;
        pub const STATIC: bool;
        pub const FINAL: bool;
        pub const SYNCRONIZED: bool;
        pub const BRIDGE: bool;
        pub const NATIVE: bool;
        const _RESERVED = 1;
        pub const ABSTRACT: bool;
        pub const STRICT: bool;
        pub const SYNTHETIC: bool;
    }
}

#[derive(Debug)]
pub struct MethodEntry {
    access_flags: AccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeEntry>
}

impl FromClassFileIter for MethodEntry {
    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        Ok(MethodEntry {
            access_flags: AccessFlags::from_bits(iter.next_u16()?),
            name_index:  iter.next_u16()?,
            descriptor_index:  iter.next_u16()?,
            attributes: AttributeEntry::from_arr(iter)?,
        })
    }
}
