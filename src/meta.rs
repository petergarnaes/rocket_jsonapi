use crate::lib::*;

pub trait Metafiable<M> {
    fn get_meta() -> M;
}
