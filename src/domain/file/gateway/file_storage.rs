use std::fs::File;
use std::io::Error;

/// A trait that defines the contract for a file storage implementation
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::Error;
/// use custodian::kernel::gateways::file_storage::{FileStorage};
///
/// struct FileStorageImpl {}
///
/// impl FileStorage for FileStorageImpl {
///    fn retrieve_file(&self, file_name: &str) -> Result<File, Error> {
///       Ok(File::open(file_name)?)
///   }
/// }
/// ```
///
/// # Remarks
///
/// This trait adapts concrete storage technologies and defines the contract for a file storage.
/// The implementation of this trait is a plugin that can be swapped out for a different implementation,
/// and is used by the `App` to retrieve a file from the file storage.
///
/// This implementation is passed to the InputSource as a dependency,
/// and is used by the InputSource to retrieve a file from the file storage.
///
pub trait FileStorage: Sync + Send {
    fn retrieve_file(&self, file_name: &str) -> Result<File, Error>;
}
