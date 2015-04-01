use db::DatabaseExt;
use model::v1 as model;
use rustless::{Nesting, Namespace};
use super::super::util::handle;

pub fn route(ns: &mut Namespace) {

    ns.get("info/:id", |endpoint| {
        handle(endpoint, |client, params| {
            let db = client.app.db();

            let id : i32 = try!(params.get("id"));
    
            model::canteen::Canteen::get_by_id(db, id)
        })
    });
}