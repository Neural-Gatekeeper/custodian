use std::fs::File;
use std::io::{Error};
use std::path::Path;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;

/// A concrete implementation of the FileStorage trait that uses the standard library to
/// interact with the file system, and retrieve a file.
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::Error;
/// use custodian::kernel::gateways::file_storage::{FileStorage};
/// use custodian::kernel::gateways::file_storage::std_fs_file_storage::StdFSFileStorage;
///
/// let file_storage = StdFSFileStorage::new();
/// let file = file_storage.retrieve_file("/some/folder/somewhere/test.txt");
/// ```
///
/// # Remarks
///
/// This implementation uses the Rust standard library to retrieve a file.
/// This implementation is used by the `InputSource` via the `AppBuilder` to
/// retrieve a file from the file storage.
///
pub struct StdFSFileStorage { }
impl StdFSFileStorage {
    pub fn new() -> StdFSFileStorage {
        StdFSFileStorage { }
    }
}
impl RetrieveFile for StdFSFileStorage {
    fn retrieve(&self, file_name: &str) -> Result<File, Error> {
        match File::open(Path::new(file_name)) {
            Ok(file) => {
                Ok(file)
            },
            Err(e) => {
                Err(Error::new(std::io::ErrorKind::NotFound, format!("Unable to retrieve the file from StdFsFileStorage: {}", e)))
            }
        }
    }
}