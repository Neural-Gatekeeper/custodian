use std::io::{Error};
use std::sync::Arc;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::io::port::input_source::{InputSource, InputSourceBuilder};
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;

pub struct CustodianApp {
    input_source: Box<dyn InputSource>,
}
impl CustodianApp {
    pub fn run(&self) {
        self.input_source.run()
    }

}

pub struct CustodianAppBuilder {
    input_source_builder: Option<Box<dyn InputSourceBuilder>>,
    retrieve_file: Option<Arc<dyn RetrieveFile>>,
    id_generator: Option<Arc<dyn IdGenerator>>,
    resource_storage: Option<Arc<dyn ResourceStorage>>,
    signature_storage: Option<Arc<dyn SignatureStorage>>,
    upload_file: Option<Arc<dyn UploadFileStorage>>,
    file_metadata: Option<Arc<dyn FileMetadataStorage>>,
}
impl CustodianAppBuilder {
    pub fn new() -> Self where Self: Sized {
        CustodianAppBuilder {
            input_source_builder: None,
            id_generator: None,
            retrieve_file: None,
            resource_storage: None,
            signature_storage: None,
            upload_file: None,
            file_metadata: None, }
    }

    pub fn using_io_channel(&mut self, builder: Box<dyn InputSourceBuilder>) -> &mut Self where Self: Sized {
        self.input_source_builder = Some(builder);
        self
    }

    pub fn with_retrieve_file(&mut self, file_storage: Arc<dyn RetrieveFile>) -> &mut Self where Self: Sized {
        self.retrieve_file = Some(Arc::clone(&file_storage));
        self
    }

    pub fn with_id_generator(&mut self, generator: Arc<dyn IdGenerator>) -> &mut Self where Self: Sized {
        self.id_generator = Some(Arc::clone(&generator));
        self
    }

    pub fn with_resource_storage(&mut self, storage: Arc<dyn ResourceStorage>) -> &mut Self where Self: Sized {
        self.resource_storage = Some(Arc::clone(&storage));
        self
    }

    pub fn with_signature_storage(&mut self, storage: Arc<dyn SignatureStorage>) -> &mut Self where Self: Sized {
        self.signature_storage = Some(Arc::clone(&storage));
        self
    }

    pub fn with_upload_file(&mut self, storage: Arc<dyn UploadFileStorage>) -> &mut Self where Self: Sized {
        self.upload_file = Some(Arc::clone(&storage));
        self
    }

    pub fn with_file_metadata(&mut self, storage: Arc<dyn FileMetadataStorage>) -> &mut Self where Self: Sized {
        self.file_metadata = Some(Arc::clone(&storage));
        self
    }

    pub fn build(&mut self) -> Result<CustodianApp, Error> {

        let server = self.input_source_builder.as_mut().unwrap()
            .with_retrieve_file(Arc::clone(&self.retrieve_file.as_ref().unwrap()))
            .with_id_generator(Arc::clone(&self.id_generator.as_ref().unwrap()))
            .with_resource_storage(Arc::clone(&self.resource_storage.as_ref().unwrap()))
            .with_signature_storage(Arc::clone(&self.signature_storage.as_ref().unwrap()))
            .with_upload_file(Arc::clone(&self.upload_file.as_ref().unwrap()))
            .with_file_metadata(Arc::clone(&self.file_metadata.as_ref().unwrap()))
            .build();
        Ok(CustodianApp { input_source: server })

    }

}