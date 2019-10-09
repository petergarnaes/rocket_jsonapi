use crate::lib::*;
use erased_serde::{Serialize as RefSerialize};

/*
pub struct LinksMeta(Box<dyn Serialize>);

impl Serialize for LinksMeta {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        (*self.0).serialize()
    }
}
*/

type Key = String;
type Url = String;

#[derive(Serialize)]
pub struct LinkObject {
    pub href: Url,
    pub meta: Box<RefSerialize>
}

impl LinkObject {
    pub fn new(href: Url, meta: Box<RefSerialize>) -> Self {
        LinkObject {href, meta }
    }
}

#[derive(Serialize)]
pub struct JsonApiLinks {

}

pub enum LinksObject {
    Url(Url),
    Object(LinkObject)
}

// TODO derive version? Maybe for the simple URL case
pub trait Linkify {
    fn get_links(&self) -> Vec<LinksObject>;
}

// TODO can we use this approach, by have some LinkifySelf trait
// TODO macro_stuff? Something like: #[derive(HaveLink, To)]
trait HaveLink<To: Linkify> {
    const KEY: Key;
}
