use std::io::{Error, ErrorKind};
use crate::domain::resource::new_resource_request::NewResourceRequest;
use crate::domain::resource::gateway::resource_storage::{Resource, ResourceStorage};
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;
use crate::kernel::values::id::Id;

/// Generates and saves a new resource to storage
pub fn new_resource(request: NewResourceRequest,
                    generator: &dyn IdGenerator,
                    storage: &dyn ResourceStorage) -> Result<Id, Error> {
    generator.generate()
    .map_err(|e| Error::new(ErrorKind::Other,
                            format!("Could not generate a ResourceId: {}", e)))
    .and_then(|resource_id| {
        match storage.save(Resource::new(resource_id, request.resource_type)) {
            Ok(_) => Ok(resource_id),
            Err(e) => Err(Error::new(ErrorKind::Other,
                                     format!("Could not save the ResourceId: {}", e)))
        }
    })
}