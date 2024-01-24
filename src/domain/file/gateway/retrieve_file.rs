use std::fs::File;
use std::io::Error;

pub trait RetrieveFile: Sync + Send {
    fn retrieve(&self, file_name: &str) -> Result<File, Error>;
}
