use crate::Data;

pub trait Device<T>: Data<T> + std::fmt::Debug {
    fn bounds(&self) -> Vec<(u32, u32)>;
}
