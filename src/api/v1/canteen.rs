use db::DatabaseExt;
use model::v1 as model;
use rustless::{Nesting, Namespace};
use super::super::util::handle;

pub fn route(ns: &mut Namespace) {

    // Get a specific canteen by ID.
    ns.get("id/:id", |endpoint| {
        handle(endpoint, |client, params| {
            // Obtain database handle and path parameter.
            let db = client.app.db();
            let id : i32 = try!(params.get("id"));

            // Execute query to fetch canteen.
            model::canteen::Canteen::get_by_id(db, id)
        })
    });
}
