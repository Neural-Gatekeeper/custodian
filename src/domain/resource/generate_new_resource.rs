use std::io::{Error, ErrorKind};
use crate::domain::resource::new_resource_request::NewResourceRequest;
use crate::domain::resource::gateway::resource_storage::{Resource, ResourceStorage};
use crate::kernel::gateways::resource_generator::ResourceGenerator;
use crate::kernel::values::resource_id::ResourceId;

/// Generates and saves a new resource to storage
pub fn generate_new_resource(request: NewResourceRequest,
                             generator: &dyn ResourceGenerator,
                             storage: &dyn ResourceStorage) -> Result<ResourceId, Error> {
    generator.generate_id()
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