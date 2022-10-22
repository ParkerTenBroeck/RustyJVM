use super::FromClassFileIter;

pub struct AttributeEntry {}

pub struct AttributeInfo {
    attribute_name_index: u16,
    // info: Vec<Info>
}

impl FromClassFileIter for AttributeEntry {
    fn from(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        todo!()
    }
}
