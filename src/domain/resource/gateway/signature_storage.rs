use std::io::Error;
use crate::kernel::values::id::Id;
use crate::kernel::values::Signature;

// todo - document with the at least to expected resutls of: 1) not vailbe, 2) resourceid already exists
pub trait SignatureStorage: Send + Sync {
    fn associate(&self, id: Id, signature: Signature) -> Result<(), Error>;
}