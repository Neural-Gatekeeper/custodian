use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};
use crate::domain::resource::gateway::resource_storage::{Resource, ResourceStorage};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::resource_id::ResourceId;
use crate::kernel::values::ResourceSignature;
use crate::kernel::values::value_object::ValueObject;

pub struct InMemoryResourceStorage {
    pub resources: Arc<Mutex<HashMap<u64, Resource>>>,
    pub signatures: Arc<Mutex<HashMap<u64, ResourceSignature>>>,
}

impl InMemoryResourceStorage {
    pub fn new() -> InMemoryResourceStorage {
        InMemoryResourceStorage {
            resources: Arc::new(Mutex::new(HashMap::new())),
            signatures: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ResourceStorage for InMemoryResourceStorage {
    fn save(&self, resource: Resource) -> Result<(), Error> {
        self.resources.lock().unwrap().insert(resource.id.value(), resource);
        Ok(())
    }
}

impl SignatureStorage for InMemoryResourceStorage {
    fn associate(&self, id: ResourceId, signature: ResourceSignature) -> Result<(), Error> {
        let mut signatures = self.signatures.lock().unwrap();
        let resources = self.resources.lock().unwrap();

        if !resources.contains_key(&id.value()) {
           return Err(Error::new(std::io::ErrorKind::Other, "ResourceId does not exist and cannot be associated with a signature."))
        };

        if signatures.contains_key(&id.value()) {
            Err(Error::new(std::io::ErrorKind::Other, "ResourceId already exists and cannot be updated"))
        } else {
            signatures.insert(id.value(), signature);
            Ok(())
        }
    }
}