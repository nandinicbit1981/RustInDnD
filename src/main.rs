extern crate rust_in_dnd;
extern crate diesel;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate mount;
extern crate staticfile;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate params;
extern crate urlencoded;

use log::LogLevel;

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
use rust_in_dnd::*;
use rust_in_dnd::models::*;
use diesel::prelude::*;
use std::process;
use params::Params;
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
use std::collections::HashMap;


#[derive(RustcEncodable, RustcDecodable)]
pub struct Character {
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
    strength_mod: f32,
    dex_mod: f32,
    con_mod: f32,
    intl_mod: f32,
    wsdm_mod: f32,
    charisma_mod: f32,
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

    use rust_in_dnd::schema::character_dnd::dsl::*;

    let connection = establish_connection();
    let results = character_dnd
        .limit(5)
        .load::<Character_DND>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.name);
        println!("----------\n");
        println!("{}", post.id);
    }
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

fn get_character(request: &mut Request) -> IronResult<Response> {

    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;
    let mut resp = Response::new();
    use rust_in_dnd::schema::character_dnd::dsl::*;
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct paramsStruct {
        id: i32
    };
    let mut nandini = 0;

    if let Ok(ref hashmap) = request.get_ref::<UrlEncodedQuery>() {
        println!("username: {:?}", hashmap.get("id").unwrap());
        nandini = hashmap.get("id").unwrap()[0].parse::<i32>().unwrap();
        println!("{:?}", nandini);
    }
    let connection = establish_connection();
    let results = character_dnd
        .filter(id.eq(nandini))
        .load::<Character_DND>(&connection)
        .expect("Error loading posts");

    let mut vec: Vec<Character> = Vec::new();

    for post in results {
        vec.push(Character {
            id: post.id,
            name: post.name,
            class: post.class,
            race: post.race,
            strength_stat: post.strength_stat,
            dextirity_stat: post.dextirity_stat,
            constitution_stat: post.constitution_stat,
            intelligence_stat: post.intelligence_stat,
            wisdom_stat: post.wisdom_stat,
            charisma_stat: post.charisma_stat,
            strength_mod: post.strength_mod,
            dex_mod: post.dex_mod,
            con_mod: post.con_mod,
            intl_mod: post.intl_mod,
            wsdm_mod: post.wsdm_mod,
            charisma_mod: post.charisma_mod,
            ac: post.ac,
        });
    }
    let payload:String = json::encode(&vec).unwrap();

    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("characters".to_string(),vec.to_json());
    resp.set_mut(Template::new("character", m.to_json())).set_mut(status::Ok);
    Ok(resp)
//    resp.set_mut(Template::new("character", m.to_json())).set_mut(status::Ok);
//    Ok(resp)
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
                strength_mod: request.strength_mod,
                dex_mod: request.dex_mod,
                con_mod: request.con_mod,
                intl_mod: request.intl_mod,
                wsdm_mod: request.wsdm_mod,
                charisma_mod: request.charisma_mod,
                ac: request.ac };

    let connection = establish_connection();
    let create_character = create_post(&connection,
                character.name,
                character.class,
                character.race,
                character.strength_stat,
                character.dextirity_stat,
                character.constitution_stat,
                character.intelligence_stat,
                character.wisdom_stat,
                character.charisma_stat,
                character.strength_mod,
                character.dex_mod,
                character.con_mod,
                character.intl_mod,
                character.wsdm_mod,
                character.charisma_mod,
                character.ac
    );

    let request: Character = Character {
        id: create_character.id,
        name: create_character.name,
        class: create_character.class,
        race: create_character.race,
        strength_stat: create_character.strength_stat,
        dextirity_stat: create_character.dextirity_stat,
        constitution_stat: create_character.constitution_stat,
        intelligence_stat: create_character.intelligence_stat,
        wisdom_stat: create_character.wisdom_stat,
        charisma_stat: create_character.charisma_stat,
        strength_mod: create_character.strength_mod,
        dex_mod: create_character.dex_mod,
        con_mod: create_character.con_mod,
        intl_mod: create_character.intl_mod,
        wsdm_mod: create_character.wsdm_mod,
        charisma_mod: create_character.charisma_mod,
        ac: create_character.ac };

    let payload = json::encode(&request).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    user_first: String,
    user_last: String
}

fn all_character(_: &mut Request) -> IronResult<Response>{
    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;

    let mut resp = Response::new();
    use rust_in_dnd::schema::character_dnd::dsl::*;

    let connection = establish_connection();
    let results = character_dnd
        .limit(5)
        .load::<Character_DND>(&connection)
        .expect("Error loading posts");

    let mut vec: Vec<Character> = Vec::new();

    for post in results {
        vec.push(Character {
            id: post.id,
            name: post.name,
            class: post.class,
            race: post.race,
            strength_stat: post.strength_stat,
            dextirity_stat: post.dextirity_stat,
            constitution_stat: post.constitution_stat,
            intelligence_stat: post.intelligence_stat,
            wisdom_stat: post.wisdom_stat,
            charisma_stat: post.charisma_stat,
            strength_mod: post.strength_mod,
            dex_mod: post.dex_mod,
            con_mod: post.con_mod,
            intl_mod: post.intl_mod,
            wsdm_mod: post.wsdm_mod,
            charisma_mod: post.charisma_mod,
            ac: post.ac,
        });
    }

    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    impl ToJson for Character {
        fn to_json(&self) -> Json {
            let mut m: BTreeMap<String, Json> = BTreeMap::new();
            m.insert("id".to_string(), self.id.to_json());
            m.insert("name".to_string(), self.name.to_json());
            m.insert("class".to_string(), self.class.to_json());
            m.insert("race".to_string(), self.race.to_json());
            m.insert("strength_stat".to_string(), self.strength_stat.to_json());
            m.insert("dextirity_stat".to_string(), self.dextirity_stat.to_json());
            m.insert("constitution_stat".to_string(), self.constitution_stat.to_json());
            m.insert("intelligence_stat".to_string(), self.intelligence_stat.to_json());
            m.insert("wisdom_stat".to_string(), self.wisdom_stat.to_json());
            m.insert("charisma_stat".to_string(), self.charisma_stat.to_json());
            m.insert("strength_mod".to_string(), self.strength_mod.to_json());
            m.insert("dex_mod".to_string(), self.dex_mod.to_json());
            m.insert("con_mod".to_string(), self.con_mod.to_json());
            m.insert("intl_mod".to_string(), self.intl_mod.to_json());
            m.insert("wsdm_mod".to_string(), self.wsdm_mod.to_json());
            m.insert("charisma_mod".to_string(), self.charisma_mod.to_json());
            m.insert("ac".to_string(), self.ac.to_json());
            m.to_json()
        }
    }

    let payload:String = json::encode(&vec).unwrap();
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("characters".to_string(),vec.to_json());
    resp.set_mut(Template::new("all", m.to_json())).set_mut(status::Ok);
    Ok(resp)
}

fn main() {

    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./src/view/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", index);
    router.get("all", all_character);
    router.get("/characterbyid", get_character);
    router.post("/character", create_character);


    let mut chain = Chain::new(router);
    chain.link_after(hbse);

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", chain)
        .mount("/assets/", Static::new(Path::new("src/assets")));


    println!("Server running at http://localhost:9000/");
    Iron::new(assets_mount).http("localhost:9000").unwrap();
}
