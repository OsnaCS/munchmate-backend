use hyper::status::StatusCode;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Json};
use rustless::errors::ErrorResponse;
use rustless::framework::endpoint::EndpointHandlerPresent;
use rustless::{Endpoint, Client, Response};
use std::str::FromStr;
use std;
use super::super::ApiError;


// Wrapper around &Json, which is used as parameter type by rustless. Getting
// values in the correct type out of &Json is verbose... this wrapper makes it
// a lot less typing (and it will do error checking).
struct LazyParam<'a> {
    param: &'a Json
}

impl<'a> LazyParam<'a> {
    // Get a parameter with name `key` and type `T`. Since all parameter are
    // strings when read from the URL, `T` needs to imlpement `FromStr`.
    pub fn get<T: FromStr>(&self, key: &str) -> Result<T, ApiError> {
        // Try to find a value with the name `key`
        let strpar = match self.param.find(key) {
            Some(val) => val.as_string().unwrap(),
            None => {
                return Err(ApiError::new(StatusCode::BadRequest,
                    format!("Missing Parameter '{}'", key)))
            },
        };

        // Try to parse the string as type `T`
        match strpar.parse() {
            Ok(val) => Ok(val),
            Err(_) => {
                let tyname = unsafe { std::intrinsics::type_name::<T>() };
                let desc = format!("Parameter '{}' cannot be parsed as '{}'",
                    key, tyname);
                Err(ApiError::detailed(StatusCode::BadRequest, desc,
                    format!("value: '{}'", strpar)))
            },
        }
    }
}


// Helper function to reduce redundant code: It wraps Endpoint::handle,
// replaces the Json params with LazyParams and replaces the closure return
// type with a more convinient return type
pub fn handle<F: 'static, R: Encodable>(ep: &mut Endpoint, handler: F)
        -> EndpointHandlerPresent
        where F: for<'a> Fn(& Client<'a>, LazyParam) ->
            Result<R, ApiError> + Sync + Send {
    // Call handle of the endpoint with wrapper closure
    ep.handle(move |client, params| {
        // Create LazyParam pack and call the handler closure with it
        let sp = LazyParam{ param: params };
        let res = handler(&client, sp);

        // Handle errors in handler: If everything was ok, the returned
        // Encodable will be encoded as json and returned as Response. If an
        // error occured, the json representation of the error will be returned
        match res {
            Ok(val) => client.text(json::encode(&val).unwrap()),
            Err(e) => {
                let resp = Response::from_string(
                    StatusCode::NotFound,
                    json::encode(&e).unwrap());
                Err(ErrorResponse { error: Box::new(e), response: Some(resp) })
            }
        }
    })
}
