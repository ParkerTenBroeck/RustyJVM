use std::collections::HashMap;

use super::class::{
    attribute::{self, AttributeInfo},
    method::MethodEntry,
    Class,
};

#[derive(Debug, Clone)]
pub enum JRTVar {
    Void,
    Char(char),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
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
    class_map: HashMap<String, Class>,
    stack: (),
    heap: (),
    code: (),
    state: (),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn insert_class(&mut self, class: Class) {
        self.class_map.insert(
            class
                .constant_pool
                .get_class_name(class.this_class)
                .unwrap()
                .into(),
            class,
        );
    }

    pub fn run_static_method(
        &mut self,
        class: &str,
        method_name: &str,
        arguments: &[JRTVar],
    ) -> Result<JRTVar, JRTError> {
        let class = self.class_map.get(class).ok_or(JRTError::ClassNotFound)?;
        let method = class
            .get_method_from_name(method_name)
            .ok_or(JRTError::MethodNotFound)?;
        for att in &method.attributes {
            if let AttributeInfo::Code { code, .. } = &att.info {
                println!("{:02X?}", code);
                return Ok(JRTVar::Void);
            }
        }
        Result::Err(JRTError::FuckyWucky)
    }
}
