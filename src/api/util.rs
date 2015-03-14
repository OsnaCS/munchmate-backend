use rustc_serialize::json::{self, Json};
use rustc_serialize::Encodable;
use rustless::framework::endpoint::EndpointHandlerPresent;
use rustless::{Endpoint, Client, Response};
use rustless::errors::ErrorResponse;
use rustless;
use iron::mime;
use std::str::FromStr;

use super::ApiError;



struct SmartParam<'a> {
    param: &'a Json
}

impl<'a> SmartParam<'a> {
    pub fn get<T: FromStr>(&self, key: &str) -> Result<T, ApiError> {
        match self.param.find(key) {
            Some(val) => match val.as_string().unwrap().parse() {
                Ok(val) => Ok(val),
                Err(_) => Err(ApiError::new("Not parse")),
            },
            None => Err(ApiError::new("Not there")),
        }
    }
}



pub fn handle<F: 'static, R: Encodable>(ep: &mut Endpoint, handler: F) 
        -> EndpointHandlerPresent 
        where F: for<'a> Fn(& Client<'a>, SmartParam) -> 
            Result<R, ApiError> + Sync + Send {
    ep.handle(move |mut client, params| {
        let sp = SmartParam{ param: params };
        let res = handler(&client, sp);
        client.set_content_type(mime::Mime(
            mime::TopLevel::Application, 
            mime::SubLevel::Json, 
            vec![(mime::Attr::Charset, mime::Value::Utf8)]
        ));
        match res {
            Ok(val) => client.text(json::encode(&val).unwrap()),
            Err(e) => {
                let resp = Response::from_string(
                    rustless::server::status::StatusCode::NotFound, 
                    json::encode(&e.desc).unwrap());
                Err(ErrorResponse { error: Box::new(e), response: Some(resp) })
            }
        }
    })
}
