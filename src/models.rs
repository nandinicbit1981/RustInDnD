
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