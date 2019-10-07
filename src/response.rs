use crate::lib::*;
use crate::info::JsonApi;
use crate::data::*;
use crate::core::data_object::create_data_object;
use crate::relationship::{HaveRelationship, AllRelationships};

fn ser<S, T, I, E, J>(
    serializer: S,
    result: &Result<JsonApiPrimaryDataObject<T, I>, E>,
    json_api: Option<J>) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Serialize + ResourceIdentifiable, I: Serialize, E: Serialize, J: Serialize {
    // TODO length aka. number of fields must be correct
    match result {
        Ok(api_result) => {
            serializer.serialize_some(&api_result)
        },
        Err(err) => {
            serializer.serialize_some(&err)
        }
    }
}

/*
fn ser<S, T, I, E, J>(
    serializer: S,
    result: &Result<JsonApiPrimaryDataObject<T, I>, E>,
    json_api: Option<J>) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Serialize + ResourceIdentifiable, I: Serialize, E: Serialize, J: Serialize {
    // TODO length aka. number of fields must be correct
    let mut state = serializer.serialize_struct("Response", 1)?;
    match result {
        Ok(api_result) => {
            // Serialize root data field
            match &api_result.data {
                PrimaryObjectType::Single(data) => {
                    state.serialize_field("data", &create_data_object(data))?;
                },
                PrimaryObjectType::Multiple(data_vec) => {
                    let data_object_vec = data_vec.iter().map(create_data_object).collect::<Vec<_>>();
                    state.serialize_field("data", &data_object_vec)?;
                }
            };
            // Serialize root links field
            match &api_result.links {
                Some(link) => {
                    // TODO handle if neither field (self or related) is set, by not parsing the links field
                    state.serialize_field("links", &link)?;
                },
                None => {}
            };
            match &api_result.included {
                Some(inclusions) => {
                    state.serialize_field("included", &inclusions)?;
                }
                None => {}
            };
        },
        Err(err) => {
            state.serialize_field("error", &err)?;
        }
    };
    // Serialize root jsonapi field
    match json_api {
        Some(json) => {
            state.serialize_field("jsonapi", &json)?;
        },
        None => {}
    };
    state.end()
}
*/

pub struct JsonApiResponse<Data: ResourceIdentifiable, Included, Error>(pub Result<JsonApiPrimaryDataObject<Data, Included>, Error>);

impl<Data, Included, Error> Serialize for JsonApiResponse<Data, Included, Error>
    where Data: Serialize + ResourceIdentifiable, Included: Serialize, Error: Serialize {
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        ser(serializer, &self.0, None::<()>)
    }
}

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
