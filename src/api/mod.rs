use rustless::{self, Api, Nesting, Versioning};
use hyper::status::StatusCode;
use rustc_serialize::{Encodable, Encoder};
use std::borrow::Borrow;
use std;
use std::num::ToPrimitive;

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



#[derive(Debug)]
struct ApiError {
    desc: String,
    code: StatusCode,
    detail: String,
}

impl ApiError {
    pub fn new(code: StatusCode, desc: String) -> ApiError {
        ApiError { desc: desc, code: code, detail: String::new() }
    }

    pub fn detailed(code: StatusCode, desc: String, detail: String) -> ApiError {
        ApiError { desc: desc, code: code, detail: detail }
    }
}

impl std::error::Error for ApiError {
    fn description(&self) -> &str {
        self.desc.borrow()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{}: {} ({})", self.code.to_u16().unwrap(), 
            self.desc, self.detail)
    }
}

impl Encodable for ApiError {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
      
        encoder.emit_struct("ApiError", 1, |encoder| {
            try!(encoder.emit_struct_field("status_code", 0, |encoder| {
                format!("{} {}",
                    self.code.to_u16().unwrap(),
                    self.code.canonical_reason().unwrap()
                ).encode(encoder)
            }));
            try!(encoder.emit_struct_field("desc", 1, |encoder| {
                self.desc.encode(encoder)
            }));
            try!(encoder.emit_struct_field("detail", 1, |encoder| {
                self.detail.encode(encoder)
            }));
            Ok(())
        })
    }
}
