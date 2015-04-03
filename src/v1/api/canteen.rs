use rustless::{Nesting, Namespace};
use super::util::handle;
use v1::db::DatabaseExt;
use v1::model::{self, Location};

pub fn route(ns: &mut Namespace) {

    // Get a specific canteen by ID.
    ns.get("id/:id", |endpoint| {
        handle(endpoint, |client, params| {
            // Obtain database handle and path parameter.
            let db = client.app.db();
            let id : i32 = try!(params.get("id"));

            // Execute query to fetch canteen.
            model::Canteen::get_by_id(db, id)
        })
    });


    ns.get("nearest", |endpoint| {
        handle(endpoint, |client, params| {
            // Obtain database handle and get parameter.
            let db = client.app.db();
            let pos = Location {
                lat: try!(params.get("lat")),
                lng: try!(params.get("lng")),
            };

            // Execute query to fetch canteens.
            model::Canteen::get_nearest(db, pos)
        })
    });
}
