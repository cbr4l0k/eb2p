use crate::domain::entities::{
    InitiativeId,
    DateYMD,
};

pub struct Inititive {
    pub id: InitiativeId,
    pub name: String,
    pub owner: String,
    pub start_date: DateYMD,
}

