use std::collections::HashMap;

use super::class::{
    attribute::{self, AttributeInfo},
    method::MethodEntry,
    Class,
};

#[derive(Debug, Clone)]
pub enum JRTVar {
    Void,
    // Char(char),
    // Byte(i8),
    // Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    // String(String),
    Object(JRTObject),
}

#[derive(Debug, Clone)]
pub struct JRTObject {
    //ya uhhhhhhhhh this ones a doozy :)
}

#[derive(Debug)]
pub enum JRTError {
    FuckyWucky,
    MethodNotFound,
    MethodNotStatic,
    ClassNotFound,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    class_list: Vec<Class>,
    class_map: HashMap<String, usize>,
    stack: Stack,
    heap: Heap,
    code: Code,
}

#[derive(Debug, Default)]
pub struct Stack {
    stack: Vec<JRTVar>,
}

#[derive(Debug, Default)]
pub struct Code {
    pc: usize,
    method_class: usize,
    raw: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Heap {}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn insert_class(&mut self, class: Class) {
        self.class_list.push(class);
        let index = self.class_list.len() - 1;
        let class = self.class_list.get_mut(index).unwrap();
        self.class_map.insert(
            class
                .constant_pool
                .get_class_name(class.this_class)
                .unwrap()
                .into(),
            index,
        );
    }

    pub fn run_static_method(
        &mut self,
        class: &str,
        method_name: &str,
        arguments: &[JRTVar],
    ) -> Result<JRTVar, JRTError> {
        let class_iid = self.class_map.get(class).ok_or(JRTError::ClassNotFound)?;
        let class = self.class_list.get(*class_iid).unwrap();
        let method = class
            .get_method_from_name(method_name)
            .ok_or(JRTError::MethodNotFound)?;
        for att in &method.attributes {
            if let AttributeInfo::Code { code, .. } = &att.info {
                //println!("{:02X?}", code);
                self.code.pc = self.code.raw.len();
                for b in code {
                    self.code.raw.push(*b);
                }
                self.code.method_class = *class_iid;
                self.run();
                return Ok(JRTVar::Void);
            }
        }
        Result::Err(JRTError::FuckyWucky)
    }

    pub fn load_method(&mut self) {}

    pub fn run(&mut self) -> Result<JRTVar, JRTError> {
        use jvm_opcodes::*;
        loop {
            let op = self.code.raw[self.code.pc];
            self.code.pc += 1;
            match op {
                NOP => {}
                GETSTATIC => {
                    let b1 = self.code.raw[self.code.pc];
                    let b2 = self.code.raw[self.code.pc + 1];
                    let index = b2 as u16 | ((b1 as u16) << 8);
                    self.code.pc += 2;
                    let c = self.class_list.get(self.code.method_class).unwrap();
                    let co = c.constant_pool.get_constant(index).unwrap();
                    todo!();
                    //this should return a Fieldref so the following wont work
                    match co {
                        crate::jvm::class::constant::ConstantPoolEntry::String { string_index } => {
                            todo!()
                        }
                        crate::jvm::class::constant::ConstantPoolEntry::Integer(i) => {
                            self.stack.stack.push(JRTVar::Int(*i))
                        }
                        crate::jvm::class::constant::ConstantPoolEntry::Float(f) => {
                            self.stack.stack.push(JRTVar::Float(*f))
                        }
                        crate::jvm::class::constant::ConstantPoolEntry::Long(l) => {
                            self.stack.stack.push(JRTVar::Long(*l))
                        }
                        crate::jvm::class::constant::ConstantPoolEntry::Double(d) => {
                            self.stack.stack.push(JRTVar::Double(*d))
                        }
                        // crate::jvm::class::constant::ConstantPoolEntry::Empty => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::Class { name_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::Fieldref { class_index, name_and_type_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::Methodref { class_index, name_and_type_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::InterfaceMethodref { class_index, name_and_type_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::NameAndType { name_index, descriptor_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::Utf8(_) => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::MethodHandle { reference_kind, reference_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::MethodType { descriptor_index } => todo!(),
                        // crate::jvm::class::constant::ConstantPoolEntry::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => todo!(),
                        _ => {
                            panic!()
                        }
                    }
                }
                LDC => {}
                INVOKEVIRTUAL => {}
                RETURN => return Ok(JRTVar::Void),
                IRETURN => return Ok(JRTVar::Int(0)),
                LRETURN => return Ok(JRTVar::Long(0)),
                FRETURN => return Ok(JRTVar::Float(0.0)),
                DRETURN => return Ok(JRTVar::Double(0.0)),
                ARETURN => return Ok(JRTVar::Object(todo!())),

                _ => {
                    panic!();
                }
            }
        }
    }
}
pub mod jvm_opcodes {

    // Constants
    pub const NOP: u8 = 0x00;
    pub const ACONST_NULL: u8 = 0x01;
    pub const ICONST_M1: u8 = 0x02;
    pub const ICONST_0: u8 = 0x03;
    pub const ICONST_1: u8 = 0x04;
    pub const ICONST_2: u8 = 0x05;
    pub const ICONST_3: u8 = 0x06;
    pub const ICONST_4: u8 = 0x07;
    pub const ICONST_5: u8 = 0x08;
    pub const LCONST_0: u8 = 0x09;
    pub const LCONST_1: u8 = 0x0a;
    pub const FCONST_0: u8 = 0x0b;
    pub const FCONST_1: u8 = 0x0c;
    pub const FCONST_2: u8 = 0x0d;
    pub const DCONST_0: u8 = 0x0e;
    pub const DCONST_1: u8 = 0x0f;
    pub const BIPUSH: u8 = 0x10;
    pub const SIPUSH: u8 = 0x11;
    pub const LDC: u8 = 0x12;
    pub const LDC_W: u8 = 0x13;
    pub const LDC2_W: u8 = 0x14;

    //Loads

    //Stores

    //Stack
    pub const POP: u8 = 0x57;
    pub const POP2: u8 = 0x58;
    pub const DUP: u8 = 0x59;
    pub const DUP_X1: u8 = 0x5a;
    pub const DUP_X2: u8 = 0x5b;
    pub const DUP2: u8 = 0x5c;
    pub const DUP2_X1: u8 = 0x5d;
    pub const DUP2_X2: u8 = 0x5e;
    pub const SWAP: u8 = 0x5f;

    //Math

    //Conversions

    //Comparison

    //References
    pub const GETSTATIC: u8 = 0xb2;
    pub const PUTSTATIC: u8 = 0xb3;
    pub const GETFIELD: u8 = 0xb4;
    pub const PUTFIELD: u8 = 0xb5;
    pub const INVOKEVIRTUAL: u8 = 0xb6;
    pub const INVOLESPECIAL: u8 = 0xb7;
    pub const INVOKESTATIC: u8 = 0xb8;
    pub const INVOKEINTERFACE: u8 = 0xb9;
    pub const INVOKEDDYNAMIC: u8 = 0xba;
    pub const NEW: u8 = 0xbb;
    pub const NEWARRAY: u8 = 0xbc;
    pub const ANEWARRAY: u8 = 0xbd;
    pub const ARRAYLENGTH: u8 = 0xbe;
    pub const ATHROW: u8 = 0xbf;
    pub const CHECKCAST: u8 = 0xc0;
    pub const INSTANCEOF: u8 = 0xc1;
    pub const MONITORENTER: u8 = 0xc2;
    pub const MONITOREXIT: u8 = 0xc3;

    //Control
    pub const GOTO: u8 = 0xa7;
    pub const JSR: u8 = 0xa8;
    pub const RET: u8 = 0xa9;
    pub const TABLESWITCH: u8 = 0xaa;
    pub const LOOKUPSWITCH: u8 = 0xab;
    pub const IRETURN: u8 = 0xac;
    pub const LRETURN: u8 = 0xad;
    pub const FRETURN: u8 = 0xae;
    pub const DRETURN: u8 = 0xaf;
    pub const ARETURN: u8 = 0xb0;
    pub const RETURN: u8 = 0xb1;

    //Extended
    pub const WIDE: u8 = 0xc4;
    pub const MULTIANEWARRAY: u8 = 0xc5;
    pub const IFNULL: u8 = 0xc6;
    pub const IFNONNULL: u8 = 0xc7;
    pub const GOTO_W: u8 = 0xc8;
    pub const JSR_W: u8 = 0xc9;

    //Reserved
    pub const BREAKPOINT: u8 = 0xca;
    pub const IMPDEP1: u8 = 0xfe;
    pub const IMPDEP2: u8 = 0xff;
}
