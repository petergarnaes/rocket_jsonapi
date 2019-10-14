use crate::lib::*;
use crate::info::JsonApi;
use crate::data::*;
use crate::core::data_object::create_data_object;
use crate::relationship::{HaveRelationship, AllRelationships};
use crate::links::Linkify;

fn ser<S, T, I, E, J>(
    serializer: S,
    result: &Result<JsonApiPrimaryDataObject<T, I>, E>,
    json_api: Option<J>) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Serialize + ResourceIdentifiable + Linkify, I: Serialize, E: Serialize, J:
Serialize {
    // TODO length aka. number of fields must be correct
    match result {
        Ok(api_result) => {
            serializer.serialize_some(&api_result)
        },
        Err(err) => {
            serializer.serialize_some(&err)
        }
    }
    // TODO handle json_api field
}

pub struct JsonApiResponse<Data: ResourceIdentifiable, Included, Error>(pub Result<JsonApiPrimaryDataObject<Data, Included>, Error>);

impl<Data, Included, Error> Serialize for JsonApiResponse<Data, Included, Error>
    where Data: Serialize + ResourceIdentifiable + Linkify, Included: Serialize, Error: Serialize {
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        ser(serializer, &self.0, None::<()>)
    }
}

/*
impl<Data, Included, Error> Serialize for JsonApiResponse<Data, Included, Error>
    where Data: Serialize + ResourceIdentifiable + AllRelationships, Included: Serialize, Error: Serialize {
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        ser(serializer, &self.0, None::<()>)
    }
}

impl<Data, Included, Error> Serialize for JsonApiResponse<Data, Included, Error>
    where Data: Serialize + ResourceIdentifiable, Included: Serialize, Error: Serialize,
          JsonApiResponse<Data, Included, Error>: JsonApi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // TODO solve Json_api field
        ser(serializer, &self.0, Some(&self.get_json_api_field()))
    }
}
*/