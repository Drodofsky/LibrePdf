#![cfg_attr(
    test,
    allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]

mod boolean;
mod name;
mod number;
mod string;
pub use boolean::*;
pub use name::*;
pub use number::*;
pub use string::*;

pub enum Object<'b> {
    Boolean(Boolean),
    Name(Name<'b>),
    Integer(Integer),
    Real(Real),
    String(String),
}

pub trait GetObj<T> {
    fn get_obj(&self) -> Option<&T>;
}

macro_rules! impl_get_obj {
    ($obj:ident) => {
        impl GetObj<$obj> for Object<'_> {
            fn get_obj(&self) -> Option<&$obj> {
                if let Object::$obj(o) = self {
                    return Some(o);
                }
                None
            }
        }
    };
}

macro_rules! impl_get_obj_lt {
    ($obj:ident) => {
        impl<'b> GetObj<$obj<'b>> for Object<'b> {
            fn get_obj(&self) -> Option<&$obj<'b>> {
                if let Object::$obj(o) = self {
                    return Some(o);
                }
                None
            }
        }
    };
}
impl_get_obj!(Boolean);
impl_get_obj_lt!(Name);
impl_get_obj!(Integer);
impl_get_obj!(Real);
impl_get_obj!(String);
