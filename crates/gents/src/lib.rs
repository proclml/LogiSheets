use std::{any::TypeId, collections::HashMap};

pub trait TS {
    fn _register(manager: &mut DescriptorManager);
    fn _ts_type() -> String;
}

pub struct DescriptorManager {
    pub descriptors: Vec<InterfaceDescriptor>,
    pub id_map: HashMap<TypeId, usize>,
}

pub struct EnumDescriptor {
    pub dependecies: Vec<usize>,
    pub fields: Vec<String>,
}

/// Describe how to generate a ts interface.
pub struct InterfaceDescriptor {
    // The index of the descriptors in the manager.
    pub dependencies: Vec<usize>,
    pub fields: Vec<FieldDescriptor>,
}

pub struct FieldDescriptor {
    pub ident: String,
    pub optional: bool,
    pub ts_ty: String,
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    #[test]
    fn type_id_test() {
        let a = TypeId::of::<Vec<usize>>();
    }
}
