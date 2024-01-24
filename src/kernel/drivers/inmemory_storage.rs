use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};
use crate::domain::resource::gateway::resource_storage::{Resource, ResourceStorage};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::id::Id;
use crate::kernel::values::Signature;
use crate::kernel::values::value_object::ValueObject;

pub struct InMemoryStorage {
    pub resources: Arc<Mutex<HashMap<u64, Resource>>>,
    pub signatures: Arc<Mutex<HashMap<u64, Signature>>>,
    pub file_metadata: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
}

impl InMemoryStorage {
    pub fn new() -> InMemoryStorage {
        InMemoryStorage {
            resources: Arc::new(Mutex::new(HashMap::new())),
            signatures: Arc::new(Mutex::new(HashMap::new())),
            file_metadata: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
impl ResourceStorage for InMemoryStorage {
    fn save(&self, resource: Resource) -> Result<(), Error> {
        self.resources.lock().unwrap().insert(resource.id.value(), resource);
        Ok(())
    }
}
impl SignatureStorage for InMemoryStorage {
    fn associate(&self, id: Id, signature: Signature) -> Result<(), Error> {
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
impl FileMetadataStorage for InMemoryStorage {
    fn save(&self, _name: String, _directory: String, _absolute_path: String) -> Result<Id, Error> {
        Id::new(0)
    }
}
impl UploadFileStorage for InMemoryStorage  {
    fn upload(&self, _directory: String, _filename: String, _data: Vec<u8>) -> Result<(), Error> {
        Ok(())
    }
}
