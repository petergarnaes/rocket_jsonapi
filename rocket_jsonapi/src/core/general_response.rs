use crate::core::data_object::JsonApiPrimaryDataObject;
use crate::error::{JsonApiError, JsonApiResponseError};
use crate::lib::*;
use crate::response::JsonApiDataResponse;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde_json::to_string as serialize;
use std::io::Cursor;

struct ResponseError<'a>(&'a Vec<JsonApiError>);

impl Serialize for ResponseError<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonApiResponseError", 1)?;
        state.serialize_field("errors", &self.0)?;
        state.end()
    }
}

pub struct JsonApiResponse<Data>(pub Status, pub Result<Data, Vec<JsonApiError>>);

impl<Data> Serialize for JsonApiResponse<Data>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    default fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.1 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&ResponseError(err)),
        }
        // TODO handle json_api field
    }
}

impl<Data> Serialize for JsonApiResponse<Vec<Data>>
where
    // TODO implement Includify + AllRelationships
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.1 {
            Ok(api_result) => serializer.serialize_some(&JsonApiPrimaryDataObject(api_result)),
            Err(err) => serializer.serialize_some(&err),
        }
        // TODO handle json_api field
    }
}

impl<'r, Data> Responder<'r> for JsonApiResponse<Data>
where
    Data: Serialize + ResourceIdentifiable + Linkify,
{
    fn respond_to(self, _request: &Request<'_>) -> Result<Response<'r>, Status> {
        // TODO improve or think about what to do in this case...
        let response = serialize(&self).map_err(|_e| Status::InternalServerError)?;

        match self.1 {
            Ok(_data) => Ok(Response::build()
                .header(ContentType::JsonApi)
                .status(self.0)
                .sized_body(Cursor::new(response))
                .finalize()),
            Err(error) => {
                let response = Response::build()
                    .header(ContentType::JsonApi)
                    .status(self.0)
                    .sized_body(Cursor::new(response))
                    .finalize();
                Ok(response)
            }
        }
    }
}
