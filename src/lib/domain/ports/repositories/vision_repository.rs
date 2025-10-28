use crate::domain::entities::{Vision, VisionId};
use anyhow::Result;

pub trait VisionRepository: Send + Sync {
    fn get(&self) -> Result<Vision>;
    fn get_by_id<T: Into<VisionId>>(&self, id: T) -> Result<Vision>;
    fn update(&mut self, vision: Vision) -> Result<()>;
} 
