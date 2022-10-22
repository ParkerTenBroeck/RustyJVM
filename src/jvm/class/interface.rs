use super::FromClassFileIter;

#[derive(Debug)]
pub struct InterfaceEntry {
    name_index: u16,
}

impl FromClassFileIter for InterfaceEntry {
    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        let c = iter.next_u8()?;
        if c != 0x07 {
            return Err(super::ClassBuilderError::InvalidConstantType(c));
        }
        Ok(InterfaceEntry {
            name_index: iter.next_u16()?,
        })
    }
}
