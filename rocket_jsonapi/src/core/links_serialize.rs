use crate::lib::*;

pub struct LinksSerialize<'a>(pub &'a Vec<Link>);

impl Serialize for LinksSerialize<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        if self.0.len() == 0 {
            return serializer.serialize_none();
        }
        let mut state = serializer.serialize_struct("LinksSerialize", self.0.len())?;
        for link in self.0 {
            match link {
                Link::Url(key, url) => {
                    state.serialize_field(key, url)?;
                }
                Link::Object(key, link_object) => {
                    state.serialize_field(key, link_object)?;
                }
            }
        }
        state.end()
    }
}
