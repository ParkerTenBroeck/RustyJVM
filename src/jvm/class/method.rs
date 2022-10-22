use super::FromClassFileIter;

pub struct MethodEntry {}

impl FromClassFileIter for MethodEntry {
    fn from(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        todo!()
    }
}
