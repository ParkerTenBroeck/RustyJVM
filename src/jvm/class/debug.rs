use crate::jvm::class::constant::ConstantPoolEntry;

use super::Class;

impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.sign_minus() {
            let super_class = if self.super_class == 0 {
                None
            } else {
                Some(self.constant_pool.get_class_name_invalid(self.super_class))
            };

            return f
                .debug_struct("ClassPrettyPrint")
                .field("minor_version", &self.major_version)
                .field("major_version", &self.major_version)
                .field("constat_pool", &ConstantNamePrint { class: self })
                .field("access_flags", &self.access_flags)
                .field(
                    "class",
                    &self.constant_pool.get_class_name_invalid(self.this_class),
                )
                .field("super", &super_class)
                // .field("interfaces", &self.interfaces)
                // .field("field_info", &self.field_info)
                // .field("method_info", &self.method_info)
                // .field("attribute_info", &self.attribute_info)
                .finish();
        } else {
            f.debug_struct("Class")
                .field("minor_version", &self.minor_version)
                .field("major_version", &self.major_version)
                .field("constant_pool", &self.constant_pool)
                .field("access_flags", &self.access_flags)
                .field("this_class", &self.this_class)
                .field("super_class", &self.super_class)
                .field("interfaces", &self.interfaces)
                .field("field_info", &self.field_info)
                .field("method_info", &self.method_info)
                .field("attribute_info", &self.attribute_info)
                .finish()
        }
    }
}

struct ConstantNamePrint<'a> {
    class: &'a Class,
}
impl<'a> std::fmt::Debug for ConstantNamePrint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for (i, constant) in self.class.constant_pool.constant_pool.iter().enumerate() {
            _ = m.entry(
                &i,
                &ConstantEntryNamePrint {
                    class: self.class,
                    constant,
                },
            )
        }
        m.finish()
    }
}

struct ConstantEntryNamePrint<'a> {
    constant: &'a ConstantPoolEntry,
    class: &'a Class,
}

impl<'a> std::fmt::Debug for ConstantEntryNamePrint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn class_type_name_helper(
            class: &Class,
            f: &mut std::fmt::Formatter<'_>,
            class_index: u16,
            name_and_type_index: u16,
            struct_name: &str,
        ) -> Result<(), std::fmt::Error> {
            let t = class.constant_pool.get_constant(name_and_type_index);
            let (name, type_str) = match t {
                Some(ConstantPoolEntry::NameAndType {
                    name_index,
                    descriptor_index,
                }) => (
                    class.constant_pool.get_const_utd8_or_invalid(*name_index),
                    class
                        .constant_pool
                        .get_const_utd8_or_invalid(*descriptor_index),
                ),
                _ => ("##NOT_NAME_AND_TYPE##", "##NOT_NAME_AND_TYPE##"),
            };
            f.debug_struct(struct_name)
                .field(
                    "class",
                    &class.constant_pool.get_class_name_invalid(class_index),
                )
                .field("name", &name)
                .field("type", &type_str)
                .finish()
        }

        match self.constant {
            ConstantPoolEntry::Empty => f.debug_struct("Empty").finish(),
            ConstantPoolEntry::Class { name_index } => f
                .debug_struct("Class")
                .field(
                    "class_name",
                    &self
                        .class
                        .constant_pool
                        .get_const_utd8_or_invalid(*name_index),
                )
                .finish(),
            ConstantPoolEntry::Fieldref {
                class_index,
                name_and_type_index,
            } => class_type_name_helper(
                self.class,
                f,
                *class_index,
                *name_and_type_index,
                "Fieldref",
            ),
            ConstantPoolEntry::Methodref {
                class_index,
                name_and_type_index,
            } => class_type_name_helper(
                self.class,
                f,
                *class_index,
                *name_and_type_index,
                "Methodref",
            ),
            ConstantPoolEntry::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => class_type_name_helper(
                self.class,
                f,
                *class_index,
                *name_and_type_index,
                "InterfaceMethodref",
            ),
            ConstantPoolEntry::String { string_index } => f
                .debug_tuple("String")
                .field(
                    &self
                        .class
                        .constant_pool
                        .get_const_utd8_or_invalid(*string_index),
                )
                .finish(),
            ConstantPoolEntry::Integer(_) => self.constant.fmt(f),
            ConstantPoolEntry::Float(_) => self.constant.fmt(f),
            ConstantPoolEntry::Long(_) => self.constant.fmt(f),
            ConstantPoolEntry::Double(_) => self.constant.fmt(f),
            ConstantPoolEntry::NameAndType {
                name_index,
                descriptor_index,
            } => f
                .debug_struct("NameAndType")
                .field(
                    "name",
                    &self
                        .class
                        .constant_pool
                        .get_const_utd8_or_invalid(*name_index),
                )
                .field(
                    "descriptor",
                    &self
                        .class
                        .constant_pool
                        .get_const_utd8_or_invalid(*descriptor_index),
                )
                .finish(),
            ConstantPoolEntry::Utf8(_) => self.constant.fmt(f),
            ConstantPoolEntry::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                let mut debug = f.debug_struct("MethodHandle");
                debug.field("reference_kind", reference_kind);
                if let Some(constant @ ConstantPoolEntry::Methodref { .. }) =
                    self.class.constant_pool.get_constant(*reference_index)
                {
                    debug.field(
                        "reference",
                        &ConstantEntryNamePrint {
                            constant,
                            class: self.class,
                        },
                    );
                } else {
                    debug.field("reference", &"##INVALID_METHOD_REFERENCE##");
                }
                debug.finish()
            }
            ConstantPoolEntry::MethodType { descriptor_index } => f
                .debug_struct("MethodType")
                .field(
                    "type",
                    &self
                        .class
                        .constant_pool
                        .get_const_utd8_or_invalid(*descriptor_index),
                )
                .finish(),
            ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                let t = self.class.constant_pool.get_constant(*name_and_type_index);
                let (name, type_str) = match t {
                    Some(ConstantPoolEntry::NameAndType {
                        name_index,
                        descriptor_index,
                    }) => (
                        self.class
                            .constant_pool
                            .get_const_utd8_or_invalid(*name_index),
                        self.class
                            .constant_pool
                            .get_const_utd8_or_invalid(*descriptor_index),
                    ),
                    _ => ("##NOT_NAME_AND_TYPE##", "##NOT_NAME_AND_TYPE##"),
                };
                f.debug_struct("InvokeDynamic")
                    .field("bootstrap_method_attr_index", bootstrap_method_attr_index)
                    .field("name", &name)
                    .field("type", &type_str)
                    .finish()
            }
        }
    }
}
