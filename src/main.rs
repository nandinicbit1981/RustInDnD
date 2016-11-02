// This is helloworld that works :)
/*


extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
v

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", set_greeting);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}*/
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use std::io::Read;
use std::io;
use rustc_serialize::json;


/// the handlers
fn index(_: &mut Request) -> IronResult<Response> {
    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;

    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("name".to_string(),"Nandini".to_json());
    m.insert("year".to_string(), "2016".to_json());


    let mut resp = Response::new();
    resp.set_mut(Template::new("index", m.to_json())).set_mut(status::Ok);
    Ok(resp)
}
#[derive(RustcEncodable, RustcDecodable)]
struct Character {
    user_first: String,
    user_last: String
}

fn setname(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let request: Character = json::decode(&payload).unwrap();
    let greeting = Character { user_first: request.user_first, user_last: request.user_last };
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
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
    router.post("/", setname);

    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    println!("Server running at http://localhost:9000/");
    Iron::new(chain).http("localhost:9000").unwrap();
}
