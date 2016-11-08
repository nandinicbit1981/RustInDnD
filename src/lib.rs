#![feature(proc_macro)]
#![recursion_limit = "64"]
#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Character_DND, newCharacter};

pub fn create_post(conn: &PgConnection,
                         name: String,
                         class: String,
                         race: String,
                         strength_stat: i32,
                         dextirity_stat : i32,
                         constitution_stat : i32,
                         intelligence_stat  : i32,
                         wisdom_stat  : i32,
                         charisma_stat  : i32,
                         strength_mod  :  f32,
                         dex_mod  :  f32,
                         con_mod  : f32,
                         intl_mod :  f32,
                         wsdm_mod  :  f32,
                         charisma_mod  :  f32,
                         ac  : i32) -> Character_DND {
    use schema::character_dnd;

    let new_character = newCharacter {
        name: name,
        class: class,
        race: race,
        strength_stat: strength_stat,
        dextirity_stat : dextirity_stat,
        constitution_stat : constitution_stat,
        intelligence_stat  : intelligence_stat,
        wisdom_stat  : wisdom_stat,
        charisma_stat  : charisma_stat,
        strength_mod  : strength_mod,
        dex_mod  : dex_mod,
        con_mod  : con_mod,
        intl_mod : intl_mod,
        wsdm_mod  : wsdm_mod,
        charisma_mod  : charisma_mod,
        ac  : ac
    };

    println!("inserting stuff");
    diesel::insert(&new_character).into(character_dnd::table)
        .get_result(conn)
        .expect("Error saving new post")
}




pub fn update_post(conn: &PgConnection,
                   id: i32,
                   name: String,
                   class: String,
                   race: String,
                   strength_stat: i32,
                   dextirity_stat : i32,
                   constitution_stat : i32,
                   intelligence_stat  : i32,
                   wisdom_stat  : i32,
                   charisma_stat  : i32,
                   strength_mod  :  f32,
                   dex_mod  :  f32,
                   con_mod  : f32,
                   intl_mod :  f32,
                   wsdm_mod  :  f32,
                   charisma_mod  :  f32,
                   ac  : i32) -> Character_DND {
    use schema::character_dnd;
    Character_DND::update(id, name, class, race, strength_stat,dextirity_stat, constitution_stat, intelligence_stat, wisdom_stat, charisma_stat,
                            strength_mod,dex_mod, con_mod, intl_mod, wsdm_mod,charisma_mod,ac).save_changes::<Character_DND>(&PgConnection)
}
