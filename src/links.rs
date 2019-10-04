use crate::lib::*;

#[derive(Serialize)]
pub struct LinkObject<M: Serialize> {
    href: String,
    meta: M
}

#[derive(Serialize)]
pub struct JsonApiLinks {

}

pub enum LinksObject<M: Serialize> {
    Url(String),
    Object(LinkObject<M>)
}

pub enum Links<MS: Serialize, MR: Serialize> {
    LinksSelf(LinksObject<MS>),
    LinksRelated(LinksObject<MR>),
    LinksSelfRelated(LinksObject<MS>, LinksObject<MR>)
}

fn serialize_links_object<S: SerializeStruct, M: Serialize>(state: &mut S, lo: &LinksObject<M>, key: &'static str) {
    match lo {
        LinksObject::Url(url) => {
            state.serialize_field(key, &url);
        },
        LinksObject::Object(l_o) => {
            state.serialize_field(key, &l_o);
        }
    };
}

impl<MS: Serialize, MR: Serialize> Serialize for Links<MS, MR> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut state = serializer.serialize_struct("LinksObject", 1)?;
        match &self {
            Links::LinksSelf(link_obj) => {
                serialize_links_object(&mut state, &link_obj, "self");
            },
            Links::LinksRelated(link_obj) => {
                serialize_links_object(&mut state, &link_obj, "related");
            },
            Links::LinksSelfRelated(ls, lr) => {
                serialize_links_object(&mut state, &ls, "self");
                serialize_links_object(&mut state, &lr, "related");
            }
        };
        state.end()
    }
}

pub trait Linkify {
    type MS: Serialize;
    type MR: Serialize;
    // TODO maybe input could be request, baseURL or something...
    fn produce_link(&self) -> Option<Links<Self::MS, Self::MR>>;
}

impl<MS: Serialize, MR: Serialize> dyn Linkify<MS = MS, MR = MR> {
    fn produce_link(&self) -> Option<Links<MS, MR>> {
        None
    }
}

pub enum LinkType {
    LinksSelf(String),
    LinksRelated(String),
    LinksSelfRelated(String, String),
    NoLink
}

pub trait Linkifiable {
    fn get_href(&self) -> LinkType;
}

pub trait LinkifyRelatedMeta<M: Serialize>: Linkifiable {
    fn get_meta(&self) -> M;
}

impl Serialize for LinkType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        unimplemented!()
    }
}

impl Serialize for dyn Linkifiable {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        unimplemented!()
    }
}

impl<M: Serialize> Serialize for dyn LinkifyRelatedMeta<M> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        unimplemented!()
    }
}
