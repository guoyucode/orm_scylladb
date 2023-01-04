use std::{fmt::Display};

use common_uu::JsonVExentd;


pub struct VecInto<T: ToString>(pub Vec<T>);
impl<'a, T: Display + Clone> From<&Vec<T>> for VecInto<T>{
    fn from(v: &Vec<T>) -> Self {
        VecInto(v.clone())
    }
}
impl<'a, T: Display> From<Vec<T>> for VecInto<T>{
    fn from(v: Vec<T>) -> Self {
        VecInto(v)
    }
}
impl<T: ToString + Clone> From<&[T]> for VecInto<T>{
    fn from(v: &[T]) -> Self {
        let v = v.to_vec();
        VecInto(v)
    }
}
static EMPTY_VEC: Vec<String> = vec![];
impl<'a> From<()> for VecInto<String> {
    fn from(_v: ()) -> Self {
        Self(EMPTY_VEC.clone())
    }
}

pub fn is_empty<T: serde::Serialize>(s: &T) -> bool {
    let obj = json!(s).as_object2().unwrap_or_default();
    for (_, v) in obj {
        if !v.is_null() {
            return false;
        }
    }
    true
}