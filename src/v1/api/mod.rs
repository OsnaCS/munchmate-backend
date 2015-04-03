use rustless::{Api, Nesting, Versioning};

mod util;
mod canteen;
mod user;

pub fn root() -> Api {
    Api::build(|api| {
        api.version("v1", Versioning::Path);

        api.resource("canteen", canteen::route);
        api.resource("user", user::route);
    })
}
