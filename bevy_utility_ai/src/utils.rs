use std::any::{type_name, TypeId};

pub fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn trim_type_name(type_name: &str) -> &str {
    if let Some((_, split_right)) = type_name.rsplit_once("::") {
        split_right
    } else {
        type_name
    }
}

pub fn type_id_of<T: 'static>(_: &T) -> TypeId {
    TypeId::of::<T>()
}
