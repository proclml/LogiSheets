use std::{any::TypeId, collections::HashMap};

pub trait TS {
    fn _register(manager: &mut DescriptorManager) -> usize;
    fn _ts_name() -> String;
}

pub struct DescriptorManager {
    pub descriptors: Vec<Descriptor>,
    pub id_map: HashMap<TypeId, usize>,
}

impl DescriptorManager {
    pub fn registry(&mut self, type_id: TypeId, descriptor: Descriptor) -> usize {
        match self.id_map.get(&type_id) {
            Some(idx) => *idx,
            None => {
                let idx = self.descriptors.len();
                self.descriptors.push(descriptor);
                idx
            }
        }
    }
}

pub enum Descriptor {
    Interface(InterfaceDescriptor),
    Enum(EnumDescriptor),
    BuiltinType(BuiltinTypeDescriptor),
    Generics(GenericDescriptor),
}

pub struct GenericDescriptor {
    pub dependencies: Vec<usize>,
    pub ts_name: String,
    pub optional: bool,
}

pub struct BuiltinTypeDescriptor {
    pub ts_name: String,
}

pub struct EnumDescriptor {
    pub dependencies: Vec<usize>,
    pub fields: Vec<String>,
    pub file_name: String,
}

/// Describe how to generate a ts interface.
pub struct InterfaceDescriptor {
    // The index of the descriptors in the manager.
    pub dependencies: Vec<usize>,
    pub fields: Vec<FieldDescriptor>,
    pub file_name: String,
}

pub struct FieldDescriptor {
    pub ident: String,
    pub optional: bool,
    pub ts_ty: String,
}

macro_rules! impl_builtin {
    ($i: ident, $l: literal) => {
        impl TS for $i {
            fn _register(manager: &mut DescriptorManager) -> usize {
                let type_id = TypeId::of::<$i>();
                let descriptor = BuiltinTypeDescriptor {
                    ts_name: $l.to_string(),
                };
                manager.registry(type_id, Descriptor::BuiltinType(descriptor))
            }

            fn _ts_name() -> String {
                $l.to_string()
            }
        }
    };
}

impl_builtin!(u8, "number");
impl_builtin!(u32, "number");
impl_builtin!(u64, "number");
impl_builtin!(i8, "number");
impl_builtin!(i32, "number");
impl_builtin!(i64, "number");
impl_builtin!(f32, "number");
impl_builtin!(f64, "number");
impl_builtin!(String, "string");
impl_builtin!(bool, "boolean");

impl<T: TS + 'static> TS for Vec<T> {
    fn _register(manager: &mut DescriptorManager) -> usize {
        let idx = T::_register(manager);
        let type_id = TypeId::of::<Self>();
        let descriptor = GenericDescriptor {
            dependencies: vec![idx],
            ts_name: Self::_ts_name(),
            optional: false,
        };
        manager.registry(type_id, Descriptor::Generics(descriptor))
    }

    fn _ts_name() -> String {
        format!("readonly {}[]", T::_ts_name())
    }
}

impl<T: TS + 'static> TS for Option<T> {
    fn _register(manager: &mut DescriptorManager) -> usize {
        let idx = T::_register(manager);
        let type_id = TypeId::of::<Self>();
        let descriptor = GenericDescriptor {
            dependencies: vec![idx],
            ts_name: Self::_ts_name(),
            optional: true,
        };
        manager.registry(type_id, Descriptor::Generics(descriptor))
    }

    fn _ts_name() -> String {
        T::_ts_name()
    }
}

impl<K, V> TS for HashMap<K, V>
where
    K: TS + 'static,
    V: TS + 'static,
{
    fn _register(manager: &mut DescriptorManager) -> usize {
        let k_dep = K::_register(manager);
        let v_dep = V::_register(manager);
        let descriptor = GenericDescriptor {
            dependencies: vec![k_dep, v_dep],
            ts_name: Self::_ts_name(),
            optional: false,
        };
        let type_id = TypeId::of::<Self>();
        manager.registry(type_id, Descriptor::Generics(descriptor))
    }

    fn _ts_name() -> String {
        format!("Map<{}, {}>", K::_ts_name(), V::_ts_name())
    }
}
