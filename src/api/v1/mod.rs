
use rustless::{Api, Nesting, Versioning};


mod canteen;

pub fn root() -> Api {
    Api::build(|api| {
        api.version("v1", Versioning::Path);

        api.resource("canteen", canteen::route);       
    })
}

