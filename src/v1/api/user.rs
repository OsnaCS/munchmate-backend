use rustless::{Nesting, Namespace};
use super::util::handle;
use v1::db::DatabaseExt;
use v1::model;

pub fn route(ns: &mut Namespace) {

    // Get a specific user by ID.
    ns.get("id/:id", |endpoint| {
        handle(endpoint, |client, params| {
            // Obtain database handle and path parameter.
            let db = client.app.db();
            let id : i64 = try!(params.get("id"));

            // Execute query to fetch canteen.
            model::User::get_by_id(db, id)
        })
    });
}
