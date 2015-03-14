use rustless;
use rustless::{Api, Nesting, Versioning};
use rustc_serialize::Encodable;
use std::borrow::Borrow;
use std;

mod canteen;
mod util;


pub fn root() -> rustless::Api {
    Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.resource("canteen", canteen::route);       
    })
}



#[derive(RustcEncodable, Debug)]
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