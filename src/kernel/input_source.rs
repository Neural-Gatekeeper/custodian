use std::sync::Arc;
use crate::domain::file::gateway::file_storage::FileStorage;
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::gateways::resource_generator::ResourceGenerator;


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

    fn with_storage_gateway(&mut self, builder: Arc<dyn FileStorage>) -> Box<dyn InputSourceBuilder>;
    fn with_resource_generator(&mut self, builder: Arc<dyn ResourceGenerator>) -> Box<dyn InputSourceBuilder>;
    fn with_resource_storage(&mut self, builder: Arc<dyn ResourceStorage>) -> Box<dyn InputSourceBuilder>;
    fn with_signature_storage(&mut self, builder: Arc<dyn SignatureStorage>) -> Box<dyn InputSourceBuilder>;
    fn build(&self) -> Box<dyn InputSource>;
}

