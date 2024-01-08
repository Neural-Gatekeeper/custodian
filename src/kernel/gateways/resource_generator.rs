use std::io::Error;
use crate::kernel::values::resource_id::ResourceId;

pub trait ResourceGenerator: Sync + Send {
    fn generate_id(&self) -> Result<ResourceId, Error>;

}