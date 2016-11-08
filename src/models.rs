#![recursion_limit = "64"]
use schema::character_dnd;

pub struct Character_DND {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub class: String,
    pub strength_stat: i32,
    pub dextirity_stat: i32,
    pub constitution_stat: i32,
    pub intelligence_stat: i32,
    pub wisdom_stat: i32,
    pub charisma_stat: i32,
    pub strength_mod: f32,
    pub dex_mod: f32,
    pub con_mod: f32,
    pub intl_mod: f32,
    pub wsdm_mod: f32,
    pub charisma_mod: f32,
    pub ac: i32,
}
//
//AsChangeset! {
//    (character_dnd)
// pub struct Character_DND {
//        pub id: i32,
//        pub name: String,
//        pub race: String,
//        pub class: String,
//        pub strength_stat: i32,
//        pub dextirity_stat: i32,
//        pub constitution_stat: i32,
//        pub intelligence_stat: i32,
//        pub wisdom_stat: i32,
//        pub charisma_stat: i32,
//        pub strength_mod: f32,
//        pub dex_mod: f32,
//        pub con_mod: f32,
//        pub intl_mod: f32,
//        pub wsdm_mod: f32,
//        pub charisma_mod: f32,
//        pub ac: i32,
//    }
//}
//
//impl Character_DND {
//    pub fn update( id: i32,
//                   name: &str,
//                   class: &str,
//                   race: &str,
//                   strength_stat: i32,
//                   dextirity_stat: i32,
//                   constitution_stat: i32,
//                   intelligence_stat: i32,
//                   wisdom_stat: i32,
//                   charisma_stat: i32,
//                   strength_mod: f32,
//                   dex_mod: f32,
//                   con_mod: f32,
//                   intl_mod: f32,
//                   wsdm_mod: f32,
//                   charisma_mod: f32,
//                   ac: i32) -> Self {
//
//        Character_DND {
//            id: id,
//            name: name.to_string(),
//            class: class.to_string(),
//            race: race.to_string(),
//            strength_stat: strength_stat,
//            dextirity_stat : dextirity_stat,
//            constitution_stat : constitution_stat,
//            intelligence_stat  : intelligence_stat,
//            wisdom_stat  : wisdom_stat,
//            charisma_stat  : charisma_stat,
//            strength_mod  : strength_mod,
//            dex_mod  : dex_mod,
//            con_mod  : con_mod,
//            intl_mod : intl_mod,
//            wsdm_mod  : wsdm_mod,
//            charisma_mod  : charisma_mod,
//            ac  : ac
//        }
//    }
//
//}

Queryable! {
       pub struct Character_DND {
        pub id: i32,
        pub name: String,
        pub race: String,
        pub class: String,
        pub strength_stat: i32,
        pub dextirity_stat: i32,
        pub constitution_stat: i32,
        pub intelligence_stat: i32,
        pub wisdom_stat: i32,
        pub charisma_stat: i32,
        pub strength_mod: f32,
        pub dex_mod: f32,
        pub con_mod: f32,
        pub intl_mod: f32,
        pub wsdm_mod: f32,
        pub charisma_mod: f32,
        pub ac: i32,
    }
}

pub struct newCharacter{
    pub name: String,
    pub class: String,
    pub race: String,
    pub strength_stat: i32,
    pub dextirity_stat : i32,
    pub constitution_stat : i32,
    pub intelligence_stat  : i32,
    pub wisdom_stat  : i32,
    pub charisma_stat  : i32,
    pub strength_mod  : f32,
    pub dex_mod  : f32,
    pub con_mod  : f32,
    pub intl_mod : f32,
    pub wsdm_mod  : f32,
    pub charisma_mod  : f32,
    pub ac  :  i32,
}

/*
#[table_name="character_dnd"]*/
Insertable! {
     (character_dnd)
     struct newCharacter {
        pub name: String,
        pub class: String,
        pub race: String,
        pub strength_stat: i32,
        pub dextirity_stat : i32,
        pub constitution_stat : i32,
        pub intelligence_stat  : i32,
        pub wisdom_stat  :  i32,
        pub charisma_stat  :  i32,
        pub strength_mod  :  f32,
        pub dex_mod  : f32,
        pub con_mod  : f32,
        pub intl_mod : f32,
        pub wsdm_mod  : f32,
        pub charisma_mod  : f32,
        pub ac  : i32
    }
}