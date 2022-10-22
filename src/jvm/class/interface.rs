use super::FromClassFileIter;

pub struct InterfaceEntry {}

impl FromClassFileIter for InterfaceEntry {
    fn from(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        todo!()
    }
}
