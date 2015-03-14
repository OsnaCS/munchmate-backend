// use rustless::{Endpoint};
use rustc_serialize::json::{self, Json};
use rustc_serialize;
use rustless::framework::endpoint::EndpointHandlerPresent;
use rustless::{Endpoint, Client, HandleResult, Nesting, Response};
use rustless::errors::ErrorResponse;
use rustless;
use std;
use std::str::FromStr;
use std::borrow::Borrow;

struct SmartParam<'a> {
    param: &'a Json
}


// impl Index<String> for SmartParam<'a> {
//     type Output = Result<(), ()>;

//     fn index(&'a self, index: &String) -> &'a Output {
//         Ok(())
//     }
// }

impl<'a> SmartParam<'a> {
    pub fn get<T: FromStr>(&'a self, key: &str) -> Result<T, ApiError> {
        match self.param.find(key) {
            Some(val) => match val.as_string().unwrap().parse() {
                Ok(val) => Ok(val),
                Err(_) => Err(ApiError::new("Not parse")),
            },
            None => Err(ApiError::new("Not there")),
        }
    }
}

pub trait SmartEndpoint {
    fn handle_smart<F: 'static, R: rustc_serialize::Encodable>(&mut self, handler: F) -> EndpointHandlerPresent 
        where F: for<'a> Fn(& Client<'a>, SmartParam) -> 
            Result<R, ApiError> + Sync + Send;
}

impl SmartEndpoint for Endpoint {
    fn handle_smart<F: 'static, R: rustc_serialize::Encodable>(&mut self, handler: F) -> EndpointHandlerPresent 
        where F: for<'a> Fn(& Client<'a>, SmartParam) -> 
            Result<R, ApiError> + Sync + Send {
        self.handle(move |mut client, params| {
            let sp = SmartParam{ param: params };
            // let r = &client;
            let res = handler(&client, sp);
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
}



#[derive(RustcDecodable, RustcEncodable, Debug)]
struct ApiError {
    desc: String,
    // code: rustless::server::status::StatusCode
}

impl ApiError {
    pub fn new(msg: &'static str) -> ApiError {
        ApiError { desc: msg.to_string() }
    }
}

impl std::error::Error for ApiError {
    fn description(&self) -> &str {
        self.desc.borrow()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}