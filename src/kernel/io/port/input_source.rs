use std::sync::Arc;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;


/// The `InputSource` trait is used to define the interface for an input source.
pub trait InputSource {
    fn run(&self);
}

/// The `InputSourceBuilder` trait is used to build an `InputSource`.
/// Any input implementation, such as s web server or a command line interface,
/// must implement this trait.  The purpose of this trait is to allow the
/// application to be built with a variety of input sources, and to provide a
/// means for the InputSource to be provided the necessary dependencies.
pub trait InputSourceBuilder {

    fn with_retrieve_file(&mut self, builder: Arc<dyn RetrieveFile>) -> Box<dyn InputSourceBuilder>;
    fn with_id_generator(&mut self, builder: Arc<dyn IdGenerator>) -> Box<dyn InputSourceBuilder>;
    fn with_resource_storage(&mut self, builder: Arc<dyn ResourceStorage>) -> Box<dyn InputSourceBuilder>;
    fn with_signature_storage(&mut self, builder: Arc<dyn SignatureStorage>) -> Box<dyn InputSourceBuilder>;
    fn with_upload_file(&mut self, builder: Arc<dyn UploadFileStorage>) -> Box<dyn InputSourceBuilder>;
    fn with_file_metadata(&mut self, builder: Arc<dyn FileMetadataStorage>) -> Box<dyn InputSourceBuilder>;
    fn build(&self) -> Box<dyn InputSource>;
}

