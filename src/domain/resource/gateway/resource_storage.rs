use std::io::Error;
use crate::domain::resource::new_resource_request::ResourceType;
use crate::kernel::values::id::Id;

pub struct Resource {
    pub id: Id,
    pub resource_type: ResourceType,
}
impl Resource {
    pub fn new(id: Id, resource_type: ResourceType) -> Resource {
        Resource {
            id,
            resource_type,
        }
    }
}

pub trait ResourceStorage: Sync + Send {
    fn save(&self, resource: Resource) -> Result<(), Error>;
}