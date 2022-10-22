use super::FromClassFileIter;

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
    String{
        string_index: u16,
    },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType{
        name_index: u16,
        descriptor_index: u16
    },
    Utf8(String),
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

impl FromClassFileIter for ConstantPoolEntry {
    fn from(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        let b = iter.next_u8()?;

        panic!()
    }
}
