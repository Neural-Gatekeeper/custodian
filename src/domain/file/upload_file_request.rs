use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct UploadFileRequest {
    pub name: String,
    pub extension: String,
    pub data: Vec<u8>,
    pub directory: String,
}
impl UploadFileRequest {
    pub fn new(filename: String, directory: String, file_data: Vec<u8>) -> Result<UploadFileRequest, Error> {
        if filename.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Filename cannot be empty"));
        }

        if directory.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Target location cannot be empty"));
        }

        if file_data.is_empty() {
            return Err(Error::new(ErrorKind::Other, "File data cannot be empty"));
        }

        let path = PathBuf::from(filename);
        let name = path.file_stem().and_then(|os_str| os_str.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|os_str| os_str.to_str()).unwrap_or("");

        Ok(UploadFileRequest {
            name: name.to_string(),
            extension: extension.to_string(),
            data: file_data,
            directory,
        })

    }

    pub fn full_name(&self) -> String {
        format!("{}.{}", self.name, self.extension).clone()
    }

    pub fn absolute_path(&self) -> String {
        format!("{}/{}", self.directory, self.full_name()).clone()
    }

}