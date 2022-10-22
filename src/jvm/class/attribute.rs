use super::FromClassFileIter;

#[derive(Debug)]
pub struct AttributeEntry {
    name_index: u16,
    info: AttributeInfo,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Raw(Vec<u8>),
    ConstantValue {
        constantvalue_indx: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        //len u32
        code: Vec<u8>,
        //len u16
        // start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16
        exception_table: Vec<()>,
        attributs: Vec<AttributeInfo>,
    },
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
}

impl FromClassFileIter for AttributeInfo {
    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        let num = iter.next_u32()?;
        let mut vec = Vec::with_capacity(num as usize);
        for _ in 0..num {
            vec.push(iter.next_u8()?);
        }
        Ok(Self::Raw(vec))
    }
}

impl FromClassFileIter for AttributeEntry {
    fn from_iter(iter: &mut super::ClassFileIter) -> Result<Self, super::ClassBuilderError> {
        Ok(AttributeEntry {
            name_index: iter.next_u16()?,
            info: AttributeInfo::from_iter(iter)?,
        })
    }
}
