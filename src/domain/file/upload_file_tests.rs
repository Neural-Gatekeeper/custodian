use std::io::{Error, ErrorKind};
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};
use crate::domain::file::upload_file::save;
use crate::domain::file::upload_file_request::UploadFileRequest;
use crate::kernel::values::id::Id;
use crate::kernel::values::Signature;
use crate::kernel::values::value_object::ValueObject;

#[test]
fn upload_file_success() {
    let request = UploadFileRequest::new("test.txt".to_string(), "/tmp".to_string(), "test".as_bytes().to_vec()).unwrap();

    match save(request, &MockUploadFileStorageSuccess {}, &MockFileMetadataStorageSuccess {}) {
        Ok(id) => assert_eq!(id, Id::new(1).unwrap().value()),
        Err(e) => assert!(false, "{}", format!("An error occurred while saving the file, and it was not expected: {:?}", e))
    }
}

#[test]
fn upload_file_storage_error() {
    let request = UploadFileRequest::new("test.txt".to_string(), "/tmp".to_string(), "test".as_bytes().to_vec()).unwrap();

    match save(request, &MockUploadFileStorageError{}, &MockFileMetadataStorageSuccess {}) {
        Ok(_) => assert!(false, "An error was expected but no error was returned."),
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::Other);
            assert_eq!(e.to_string(), "The file could not be saved: MockUploadFileStorageError - failed");

        }
    }
}

#[test]
fn upload_file_metadata_storage_error() {
    let request = UploadFileRequest::new("test.txt".to_string(), "/tmp".to_string(), "test".as_bytes().to_vec()).unwrap();

    match save(request, &MockUploadFileStorageSuccess{}, &MockFileMetadataStorageError {}) {
        Ok(_) => assert!(false, "An error was expected but no error was returned."),
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::Other);
            assert_eq!(e.to_string(), "The file metadata could not be saved: MockFileMetadataStorageError - failed");

        }
    }
}


struct MockUploadFileStorageSuccess {}
impl UploadFileStorage for MockUploadFileStorageSuccess {
    fn upload(&self, _directory: String, _filename: String, _data: Vec<u8>) -> Result<(), Error> {
        Ok(())
    }
}

struct MockUploadFileStorageError {}
impl UploadFileStorage for MockUploadFileStorageError {
    fn upload(&self, _directory: String, _filename: String, _data: Vec<u8>) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, "MockUploadFileStorageError - failed"))
    }
}

struct MockFileMetadataStorageSuccess {}
impl FileMetadataStorage for MockFileMetadataStorageSuccess {
    fn save(&self, _filename: String, _location: String, _signature: Signature) -> Result<Id, Error> {
        Id::new(1)
    }
}

struct  MockFileMetadataStorageError {}
impl FileMetadataStorage for MockFileMetadataStorageError {
    fn save(&self, _filename: String, _location: String, _signature: Signature) -> Result<Id, Error> {
        Err(Error::new(std::io::ErrorKind::Other, "MockFileMetadataStorageError - failed"))
    }
}