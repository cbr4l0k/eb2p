use chrono::NaiveDate;
use crate::domain::entities::InitiativeId;

pub struct Inititive {
    pub id: InitiativeId,
    pub name: String,
    pub owner: String,
    pub start_date: NaiveDate,
}

impl Inititive {
    pub fn new<T: Into<String>>(
        name: T,
        owner: T,
        start_date: NaiveDate,
    ) -> Self {
        Self {
            id: InitiativeId::new(),
            name: name.into(),
            owner: owner.into(),
            start_date,
        }
    }

    pub fn update_name<T: Into<String>>(&mut self, new_name: T) {
        self.name = new_name.into();
    }

    pub fn update_owner<T: Into<String>>(&mut self, new_owner: T) {
        self.owner = new_owner.into();
    }

    pub fn update_start_date(&mut self, new_start_date: NaiveDate) {
        self.start_date = new_start_date;
    }
}
