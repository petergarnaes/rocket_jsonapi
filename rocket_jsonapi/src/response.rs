use crate::core::data_object::create_data_object;
use crate::data::*;
use crate::lib::*;
use crate::links::Linkify;

fn ser<S, T, E, J>(
    serializer: S,
    result: &Result<T, E>,
    json_api: Option<J>,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + ResourceIdentifiable + Linkify,
    E: Serialize,
    J: Serialize,
{
    match result {
        Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
        Err(err) => serializer.serialize_some(&err),
    }
    // TODO handle json_api field
}

pub struct JsonApiResponse<Data, Error>(pub Result<Data, Error>)
where
    Data: ResourceIdentifiable;

impl<Data, Error> Serialize for JsonApiResponse<Data, Error>
where
    // TODO implement Includify
    Data: Serialize + ResourceIdentifiable + Linkify,
    Error: Serialize,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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
