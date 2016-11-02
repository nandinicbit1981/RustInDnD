extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate handlebars_iron as hbs;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use hbs::{Template, HandlebarsEngine, DirectorySource};

#[cfg(feature = "serde_type")]
mod data {
    pub fn make_data() -> BTreeMap<String, Value> {
        let mut data = BTreeMap::new();
        data.insert("year".to_string(), value::to_value(&"2016"));
        data.insert("name".to_string(), value::to_value(&"Nandini"));
    }
}

/// the handlers
fn index(_: &mut Request) -> IronResult<Response> {
    use data::*;
    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::new("some/path/hello", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./view/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", index);

    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    chain.link_after(ErrorReporter);
    println!("Server running at http://localhost:9000/");
    Iron::new(chain).http("localhost:9000").unwrap();
}