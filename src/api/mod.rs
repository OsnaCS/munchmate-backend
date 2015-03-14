use rustless;
use rustless::{Api, Nesting, Versioning};

mod canteen;


pub fn root() -> rustless::Api {
    Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.resource("canteen", canteen::route);       
    })
}