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
extern crate mount;
extern crate staticfile;
use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use std::io::Read;
use std::io;
use rustc_serialize::json;
use mount::*;
use std::path::Path;

use staticfile::Static;

#[derive(RustcEncodable, RustcDecodable)]
struct Character {
    id: i32,
    name: String,
    class: String,
    race: String,
    strength_stat: i32,
    dextirity_stat: i32,
    constitution_stat: i32,
    intelligence_stat: i32,
    wisdom_stat: i32,
    charisma_stat: i32,
    strength_mod: i32,
    dex_mod: i32,
    con_mod: i32,
    intl_mod: i32,
    wsdm_mod: i32,
    charisma_mod: i32,
    ac: i32
}

enum Race {
    Human,
    Dwarf,
    Elf,
    HalfElf
}

struct Stats {
    stat: i32,
    modifier: f32
}

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


fn setname(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let request: Greeting = json::decode(&payload).unwrap();
    let greeting = Greeting { user_first: request.user_first, user_last: request.user_last };
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn create_character(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let request: Character = json::decode(&payload).unwrap();
    let character = Character {
                id: request.id,
                name: request.name,
                class: request.class,
                race: request.race,
                strength_stat: request.strength_stat,
                dextirity_stat: request.dextirity_stat,
                constitution_stat: request.constitution_stat,
                intelligence_stat: request.intelligence_stat,
                wisdom_stat: request.wisdom_stat,
                charisma_stat: request.charisma_stat,
                strength_mod: request.strength_stat,
                dex_mod: request.dex_mod,
                con_mod: request.con_mod,
                intl_mod: request.intl_mod,
                wsdm_mod: request.wsdm_mod,
                charisma_mod: request.charisma_mod,
                ac: request.ac };
    let payload = json::encode(&character).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    user_first: String,
    user_last: String
}

/*
fn create_character(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();

    let character: Character = json::decode(&payload).unwrap();
    let payload = json::encode(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}
*/

fn main() {

    //router.post("/character", create_character);

    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./src/view/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut chain = Chain::new(index);
    chain.link_after(hbse);

    let mut router = Router::new();
    router.get("/", chain);
    router.post("/character", create_character);

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/assets/", Static::new(Path::new("src/assets")));


    println!("Server running at http://localhost:9000/");
    Iron::new(assets_mount).http("localhost:9000").unwrap();
}
