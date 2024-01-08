use std::io::{Error};
use std::sync::Arc;
use crate::domain::file::gateway::file_storage::FileStorage;
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::input_source::{InputSource, InputSourceBuilder};
use crate::kernel::gateways::resource_generator::ResourceGenerator;

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
    file_storage: Option<Arc<dyn FileStorage>>,
    resource_generator: Option<Arc<dyn ResourceGenerator>>,
    resource_storage: Option<Arc<dyn ResourceStorage>>,
    signature_storage: Option<Arc<dyn SignatureStorage>>,
}
impl CustodianAppBuilder {
    pub fn new() -> Self where Self: Sized {
        CustodianAppBuilder {
            input_source_builder: None,
            resource_generator: None,
            file_storage: None,
            resource_storage: None,
            signature_storage: None }
    }

    pub fn using_io_channel(&mut self, builder: Box<dyn InputSourceBuilder>) -> &mut Self where Self: Sized {
        self.input_source_builder = Some(builder);
        self
    }

    pub fn with_file_storage(&mut self, file_storage: Arc<dyn FileStorage>) -> &mut Self where Self: Sized {
        self.file_storage = Some(Arc::clone(&file_storage));
        self
    }

    pub fn with_resource_generator(&mut self, generator: Arc<dyn ResourceGenerator>) -> &mut Self where Self: Sized {
        self.resource_generator = Some(Arc::clone(&generator));
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

    pub fn build(&mut self) -> Result<CustodianApp, Error> {

        let server = self.input_source_builder.as_mut().unwrap()
            .with_storage_gateway(Arc::clone(&self.file_storage.as_ref().unwrap()))
            .with_resource_generator(Arc::clone(&self.resource_generator.as_ref().unwrap()))
            .with_resource_storage(Arc::clone(&self.resource_storage.as_ref().unwrap()))
            .with_signature_storage(Arc::clone(&self.signature_storage.as_ref().unwrap()))
            .build();
        Ok(CustodianApp { input_source: server })

    }

}