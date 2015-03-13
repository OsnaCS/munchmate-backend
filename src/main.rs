#![feature(plugin)]
// #![allow(dead_code)]


extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate valico;

use iron::Iron;
use rustless::{
    Application, Api, Nesting, Versioning
};
use std::str::FromStr;
use std::borrow::Borrow;



fn main() {

    let port = match fetch_env() {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };

    let api = Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.get("info/:id", |endpoint| {
            endpoint.handle(|client, params| {
                client.text(params.find("id")
                    .unwrap().as_string().unwrap().to_string())
            })
        });
       
    });

    let app = Application::new(api);

    println!("Listening on port {}", port);
    Iron::new(app).http(("localhost", port)).unwrap();
}


fn fetch_env() -> Result<u16, String> {
    use std::env;

    let port = match env::var("PORT") {
        Err(_) => return Err("Env-Var 'PORT' is not set!".to_string()),
        Ok(val) => match FromStr::from_str(val.borrow()) {
            Err(_) => return Err("Env-Var 'PORT' is not an integer!"
                .to_string()),
            Ok(p) => p,
        },
    };

    Ok(port)
}