use std::io::Error;
use crate::kernel::values::resource_id::ResourceId;
use crate::kernel::values::ResourceSignature;

// todo - document with the at least to expected resutls of: 1) not vailbe, 2) resourceid already exists
pub trait SignatureStorage: Send + Sync {
    fn associate(&self, id: ResourceId, signature: ResourceSignature) -> Result<(), Error>;
}