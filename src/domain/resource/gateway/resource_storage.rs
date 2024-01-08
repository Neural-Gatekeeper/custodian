use std::io::Error;
use crate::domain::resource::new_resource_request::ResourceType;
use crate::kernel::values::resource_id::ResourceId;

pub struct Resource {
    pub id: ResourceId,
    pub resource_type: ResourceType,
}
impl Resource {
    pub fn new(id: ResourceId, resource_type: ResourceType) -> Resource {
        Resource {
            id,
            resource_type,
        }
    }
}

pub trait ResourceStorage: Sync + Send {
    fn save(&self, resource: Resource) -> Result<(), Error>;
}