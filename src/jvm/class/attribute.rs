use super::{ClassBuilderError, ClassFileIter, FromClassFileIter};

#[derive(Debug)]
pub struct AttributeEntry {
    pub name_index: u16,
    pub info: AttributeInfo,
}
impl AttributeEntry {
    pub fn parse(&mut self, constants: &mut super::constant::ConstantPool) {
        if let AttributeInfo::Raw(vec) = &self.info {
            let name = constants.get_const_utd8(self.name_index).unwrap_or("");
            match name {
                "Code" => {
                    let mut data = ClassFileIter::new(vec);

                    let res: Result<AttributeInfo, ClassBuilderError> = (|| {
                        Ok(AttributeInfo::Code {
                            max_stack: data.next_u16()?,
                            max_locals: data.next_u16()?,
                            code: {
                                let tmp = data.next_u32()? as usize;
                                data.next_n_u8(tmp)?
                            },
                            // exception_table: (),
                            // attributs: ()
                        })
                    })();
                    if let Ok(res) = res {
                        self.info = res;
                    }
                }
                "dflkadflkfda" => {
                    panic!(); //so clippy doesnt complain for now :)
                }
                _ => {
                    //for now we ignore anything we dont know
                }
            }
        }
    }
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
        // exception_table: Vec<()>,
        // attributs: Vec<AttributeInfo>,
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
