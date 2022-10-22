use super::{attribute::AttributeEntry, FromClassFileIter};

#[derive(Debug)]
pub struct FieldEntry {
    access_flags: AccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: Vec<AttributeEntry>,
}

impl FromClassFileIter for FieldEntry {
    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        Ok(FieldEntry {
            access_flags: AccessFlags::from_bits(iter.next_u16()?),
            name_index: iter.next_u16()?,
            descriptor_index: iter.next_u16()?,
            attributes_count: AttributeEntry::from_arr(iter)?,
        })
    }
}

mycelium_bitfield::bitfield! {
    /// Bitfield types can have doc comments.
    #[derive(Eq, PartialEq)] // ...and attributes
    pub struct AccessFlags<u16> {
        pub const PUBLIC: bool;
        pub const PRIVATE: bool;
        pub const PROTECTED: bool;
        pub const STATIC: bool;
        pub const FINAL: bool;
        pub const _RESERVED = 1;
        pub const VOLATILE: bool;
        pub const TRANSIENT: bool;

        const _RESERVED2 = 4;

        pub const SYNTHETIC: bool;
        pub const _RESERVED3 = 1;
        pub const ENUM: bool;
    }
}
