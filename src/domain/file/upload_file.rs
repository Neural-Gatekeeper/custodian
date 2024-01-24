use std::io::{Error, ErrorKind};
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};
use crate::domain::file::upload_file_request::UploadFileRequest;
use crate::kernel::values::id::Id;

// todo - update this to take a SignatureStrategy and create the signature as part of this use case
// todo - create  FileMetadata struct which contains the signature, filename, and directory
pub fn save(request: UploadFileRequest,
            file_store: &dyn UploadFileStorage,
            metadata_store: &dyn FileMetadataStorage) -> Result<Id, Error> {

    file_store.upload(request.directory.clone(), request.full_name(), request.data.clone())
        .map_err(|e|
            Error::new(ErrorKind::Other, format!("The file could not be saved: {}", e.to_string())))?;

    // todo - update this to return the signature, not the id
    metadata_store.save(request.name.clone(), request.directory.clone(), request.absolute_path())
        .map_err(|e|
            Error::new(ErrorKind::Other, format!("The file metadata could not be saved: {}", e.to_string())))
    }