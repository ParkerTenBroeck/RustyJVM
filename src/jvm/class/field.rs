use super::FromClassFileIter;

pub struct FieldEntry {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
}

impl FromClassFileIter for FieldEntry {
    fn from(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        todo!()
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
