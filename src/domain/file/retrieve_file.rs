use std::fs::File;
use std::io::Error;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;

/// Retrieve a file from the file storage
///
/// # Arguments
///
/// * `file_path` - The full path of the file to retrieve
/// * `storage` - The file storage to retrieve the file from
///
/// # Returns
///
/// * `Result<File, Error>` - The file retrieved from the file storage
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::Error;
/// use std::sync::Arc;
/// use custodian::kernel::gateways::file_storage::{FileStorage};
/// use custodian::domain::retrieve_file::retrieve_from_filestore::{retrieve_file_from};
///
/// let file_storage = Arc::new(FileStorage::new());
/// let file = retrieve_file_from("/some/folder/somewhere/test.txt", &file_storage);
/// ```
///
/// # Remarks
///
/// This function is a domain use-case used to retrieve a file from the file storage
///
pub fn retrieve_file_from(file_path: &str, storage: &dyn RetrieveFile) -> Result<File, Error> {
    storage.retrieve(file_path)
}