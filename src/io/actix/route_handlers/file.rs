use std::io::Error;
use std::path::PathBuf;
use std::sync::Arc;
use actix_files::NamedFile;
use actix_web::{HttpRequest};
use actix_web::web::Data;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;
use crate::domain::file::retrieve_file::retrieve_file_from;
use crate::io::actix::route_handlers::utils::get_query_param_value;

pub async fn get(req: HttpRequest, file_storage: Data<Arc<dyn RetrieveFile>>) -> Result<NamedFile, Error> {
    let file_path = get_query_param_value(req.uri().to_string(), "file_path").unwrap_or("".to_string());
    match retrieve_file_from(&file_path, file_storage.get_ref().as_ref()) {
        Ok(file) => {
            match NamedFile::from_file(file, PathBuf::from(file_path).as_path()) {
                Ok(named_file) => Ok(named_file),
                Err(e) => Err(Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", e)))
            }
        },
        Err(e) => Err(Error::new(std::io::ErrorKind::NotFound, format!("File not found: {}", e)))
    }
}





