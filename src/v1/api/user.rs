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

            // Execute query to fetch user.
            model::User::get_by_id(db, id)
        })
    });

    ns.get("contacts", |endpoint| {
        handle(endpoint, |client, params| {
            // Obtain database handle and get parameter.
            let db = client.app.db();
            let myid : i64 = try!(params.get("myid"));

            // Execute query to fetch contacts.
            model::User::contacts(db, myid)
        })
    });

    ns.get("login", |endpoint| {
        handle(endpoint, |client, params| {
            let db = client.app.db();
            let username : String = try!(params.get("username"));
            let auth_key : String = try!(params.get("auth_key"));

            model::User::login(db, username, auth_key)
        })
    });
}
