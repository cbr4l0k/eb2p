use crate::domain::entities::{Inititive, InitiativeId}; 
use anyhow::Result;


pub trait InitiativeRepository: Send + Sync {
    fn get_all(&self) -> Result<Vec<Inititive>>;
    fn get_by_id<T: Into<InitiativeId>>(&self, id: T) -> Result<Option<Inititive>>;
    fn create(&mut self, initiative: Inititive) -> Result<Inititive>;
    fn update<T: Into<InitiativeId>>(&mut self, id: T, initiative: Inititive) -> Result<()>;
    fn delete<T: Into<InitiativeId>>(&mut self, id: T) -> Result<()>;
}
