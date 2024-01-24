use std::io::{Error, ErrorKind};
use std::sync::Arc;
use crate::domain::resource::new_resource::new_resource;
use crate::domain::resource::new_resource_request::NewResourceRequest;
use crate::domain::resource::gateway::resource_storage::{Resource, ResourceStorage};
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;
use crate::kernel::values::id::Id;

/// Ensure that the resource id is generated successfully when the
/// ResourceGenerator returns a valid ResourceId.
#[test]
fn generate_resource_id_successfully() {
    let request = NewResourceRequest::new("model").unwrap();
    let generator = Arc::new(OneResourceGenerator{}) as Arc<dyn IdGenerator>;
    let storage = Arc::new(MockResourceStorage{}) as Arc<dyn ResourceStorage>;
    let result = new_resource(request, generator.as_ref(), storage.as_ref());
    match result {
        Ok(resource_id) => {
            assert_eq!(resource_id, 1) }
        Err(_) => {
            assert!(false, "Should not have failed");
        }
    }
}

/// Ensure that the resource id is not generated when the
/// ResourceGenerator returns an error, and the error is
/// returned.
///
/// The error must start with, "Could not generate a ResourceId: ", and
/// then a message from the gateway should be appended.
#[test]
fn generate_resource_id_returns_error() {
    let request = NewResourceRequest::new("model").unwrap();
    let generator = Arc::new(ErrorResourceGenerator{}) as Arc<dyn IdGenerator>;
    let storage = Arc::new(MockResourceStorage{}) as Arc<dyn ResourceStorage>;
    let result = new_resource(request, generator.as_ref(), storage.as_ref());
    match result {
        Ok(_) => {
            assert!(false, "This should have failed, but it did not."); }
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::Other);
            assert_eq!(e.to_string(), "Could not generate a ResourceId: Error from the ErrorResourceGenerator");
        }
    }
}

struct OneResourceGenerator { }
impl IdGenerator for OneResourceGenerator {
    fn generate(&self) -> Result<Id, Error> {
        Ok( Id::new(1).unwrap() )
    }
}

struct ErrorResourceGenerator { }
impl IdGenerator for ErrorResourceGenerator {
    fn generate(&self) -> Result<Id, Error> {
        Err(
            Error::new(ErrorKind::Other,
                       "Error from the ErrorResourceGenerator")
        )
    }
}

struct MockResourceStorage { }
impl ResourceStorage for MockResourceStorage {
    fn save(&self, _resource: Resource) -> Result<(), Error> {
          Ok(())
    }
}

struct ErrorResourceStorage { }
impl ResourceStorage for ErrorResourceStorage {
    fn save(&self, _resource: Resource) -> Result<(), Error> {
        Err(
            Error::new(ErrorKind::Other,
                       "Error from the ErrorResourceStorage")
        )
    }
}
