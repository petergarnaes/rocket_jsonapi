use crate::core::data_object::create_data_object;
use crate::lib::*;
use crate::links::Linkify;

pub trait ResourceIdentifiable {
    type IdType: ToString;

    fn get_type(&self) -> &'static str;
    fn get_id(&self) -> &Self::IdType;
}

// Newtype to customize parsing of ResourceIdentifiable, so we don't need to allocate a new data
// structure
struct ResourceIdent<'a, R>(&'a R);

impl<'a, R> Serialize for ResourceIdent<'a, R>
where
    R: Serialize + ResourceIdentifiable,
{
    default fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceIdentifier", 3)?;
        state.serialize_field("id", &self.0.get_id().to_string())?;
        state.serialize_field("type", self.0.get_type())?;
        state.serialize_field("attributes", &self.0)?;
        state.end()
    }
}

// Specialized case where we can simply read the Id, without having to convert it to a string first
impl<'a, R> Serialize for ResourceIdent<'a, R>
where
    R: Serialize + ResourceIdentifiable<IdType = String>,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResourceIdentifier", 3)?;
        // Specialized part, here we simply read the Id value, no conversion needed
        state.serialize_field("id", &self.0.get_id())?;
        state.serialize_field("type", self.0.get_type())?;
        state.serialize_field("attributes", &self.0)?;
        state.end()
    }
}

// TODO maybe move to core, and hide for user
#[derive(Serialize)]
pub struct ResourceIdentifier {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: &'static str,
}

impl ResourceIdentifier {
    pub fn create_identifier<T: ResourceIdentifiable>(resource: &T) -> Self {
        ResourceIdentifier {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl<T: ResourceIdentifiable> From<&T> for ResourceIdentifier {
    fn from(resource: &T) -> Self {
        ResourceIdentifier {
            id: resource.get_id().to_string(),
            object_type: resource.get_type(),
        }
    }
}

impl ResourceIdentifiable for ResourceIdentifier {
    type IdType = String;

    fn get_type(&self) -> &'static str {
        self.object_type
    }

    fn get_id(&self) -> &String {
        &self.id
    }
}

pub enum ResourceObjectType<Data> {
    Single(Data),
    Multiple(Vec<Data>),
    NoResource,
}

pub enum PrimaryObjectType<Data: ResourceIdentifiable> {
    Single(Data),
    Multiple(Vec<Data>),
}

// Struct for data, will be parsed correctly
// TODO move to core
pub struct JsonApiPrimaryDataObject<'a, Data>(pub &'a Data);

// TODO could we make this the only implementation, and make default implementation for Linkify and
// whatever I deside on for Relationships? Since these fields are optional it would make sense to
// provide the user the option to return nothing, based on the data object (ie. NoLink enum and
// corresponding in relationship). It would also make gradual and modular implementation possible,
// ie. only implement Linkify, or only relationship, etc.
impl<'a, Data> Serialize for JsonApiPrimaryDataObject<'a, Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 3)?;
        state.serialize_field("data", &ResourceIdent(self.0))?;
        let links = Data::get_links();
        match links.len() {
            0 => {
                // TODO do not parse the links field
            }
            _ => {
                // TODO parse each element as a nested object in parent links object, use provided key
            }
        }
        // TODO Includify and Relationships
        state.end()
    }
}

struct JsonApiPrimaryDataObjectArray<'a, Data>(&'a Vec<Data>);

impl<'a, Data> Serialize for JsonApiPrimaryDataObjectArray<'a, Data>
where
    Data: Serialize + ResourceIdentifiable,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for res in self.0 {
            seq.serialize_element(&ResourceIdent(res))?;
        }
        seq.end()
    }
}

impl<'a, Data> Serialize for JsonApiPrimaryDataObject<'a, Vec<Data>>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiPrimaryDataObject", 3)?;
        state.serialize_field("data", &JsonApiPrimaryDataObjectArray(self.0))?;
        let links = Data::get_links();
        match links.len() {
            0 => {
                // TODO do not parse the links field
            }
            _ => {
                // TODO parse each element as a nested object in parent links object, use provided key
            }
        }
        // TODO Includify and Relationships
        state.end()
    }
}
