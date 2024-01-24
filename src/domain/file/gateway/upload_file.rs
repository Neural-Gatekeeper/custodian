use std::io::Error;
use crate::kernel::values::id::Id;
use crate::kernel::values::Signature;

pub trait UploadFileStorage: Sync + Send {
    fn upload(&self, directory: String, filename: String, data: Vec<u8>) -> Result<(), Error>;
}

// todo - update with a FileMeatadata struct
pub trait FileMetadataStorage: Sync + Send {
    fn save(&self, filename: String, location: String, signature: Signature) -> Result<Id, Error>;
}