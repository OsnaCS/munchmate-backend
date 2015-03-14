#![feature(plugin)]
#![feature(core)]

extern crate iron;
extern crate openssl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rustless;
extern crate typemap;

use iron::Iron;
use rustless::Application;
use std::borrow::Borrow;

mod db;
mod api;
mod util;


// If the Option obtained by $x is None, it will return, otherwise return Some
macro_rules! try_or_return {
    ($x:expr) => {
        match $x {
            Some(result) => result,
            None => return,
        }
    };
}

fn main() {
    // Try to get all needed env-vars
    let (port, db_url) = try_or_return!(fetch_env());
    
    // Try to create database pool and connect to database
    let db_pool = try_or_return!(db::setup(db_url.borrow(), 1));

    // Create rustless application with root api from module api
    let mut app = Application::new(api::root());

    // Insert the database pool into the typemap to make it available for the
    // api endpoints
    app.ext.insert::<db::AppDB>(db_pool);

    // Start HTTP server on the given port
    println!("Listening on port {}", port);
    Iron::new(app).http(("localhost", port)).unwrap();
}


// Fetches all needed environment variables
fn fetch_env() -> Option<(u16, String)> {
    use std::str::FromStr;
    use std::env;

    // Try to get 'PORT' and convert it to u16
    let port = match env::var("PORT") {
        Err(_) => {
            println!("Environment variable 'PORT' is not set!");
            return None;
        },
        Ok(val) => match FromStr::from_str(val.borrow()) {
            Err(_) => {
                println!("Environment variable 'PORT' is not an integer!");
                return None;
            },
            Ok(p) => p,
        },
    };

    // Try to get the database connection string
    let db_url = match env::var("DATABASE_URL") {
        Err(_) => {
            println!("Environment variable 'DATABASE_URL' is not set!");
            return None;
        },
        Ok(val) => val,
    };

    Some((port, db_url))
}