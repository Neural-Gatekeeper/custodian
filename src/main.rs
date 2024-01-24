use std::sync::Arc;
use crate::application::custodian;
use domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::file::gateway::retrieve_file::RetrieveFile;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::io::actix::actix_web_server::ActixWebServerBuilder;
use crate::io::std::std_fs_filestore::StdFSFileStorage;
use crate::kernel::drivers::atomic_id_generator::AtomicIdGenerator;
use crate::kernel::drivers::inmemory_storage::InMemoryStorage;
use kernel::usecase::id::id_generator_gateway::IdGenerator;
use crate::domain::file::gateway::upload_file::{FileMetadataStorage, UploadFileStorage};

mod application;
mod domain;
mod io;
mod kernel;

fn main() {

    // todo - LEFT OFF - implement #3 and create an file-storage gateway for #3
    // todo - LEFT OFF - need to modify the upload uscase to take in a signature strategy, and crfeate a filemetdata object
    // 1) upload file, 2) create a resoruce id, 3) get the sha of the data, 4) associate the sha with the resource id
    // todo - 3 - create a usecase to associate a file/ file-signature with a with a resource-id.
        // update the /file to be a POST and provide the resource-id as a form-data param.
    // todo - pre 4 - update how signatures work and provide a facility on for a SignatureStrategy with validates a signature.
    // todo - pre 4 - semantically refactor resource_generator to id_generator and move it to the kernel
    // todo - 4 - create a usecase to upload a file to the filetorage
        // when you upload a file, give it a resource id as well as take the signature, and associate the signature with the resrouce id
    // todo - 5 create a usecase to get a representation of all the files associated with a specific resoruce

    let atomic_resource_generator = Arc::new(AtomicIdGenerator::new());
    let id_generator = Arc::clone(&atomic_resource_generator) as Arc<dyn IdGenerator>;

    let std_fs_file_storage= Arc::new(StdFSFileStorage::new());
    let retrieve_file = Arc::clone(&std_fs_file_storage) as Arc<dyn RetrieveFile>;
    // todo - add UploadFile to StdFSFileStorage

    let in_memory_resource_storage = Arc::new(InMemoryStorage::new());
    let resource_storage = Arc::clone(&in_memory_resource_storage) as Arc<dyn ResourceStorage>;
    let signature_storage = Arc::clone(&in_memory_resource_storage) as Arc<dyn SignatureStorage>;
    let file_metadata = Arc::clone(&in_memory_resource_storage) as Arc<dyn FileMetadataStorage>;
    let upload_file_storage = Arc::clone(&in_memory_resource_storage) as Arc<dyn UploadFileStorage>;
    // todo - add FileMetadataStorage to InMemoryStorage

    let builder_result = custodian::CustodianAppBuilder::new()
        .using_io_channel(ActixWebServerBuilder::new("127.0.0.1", 8000))
        .with_retrieve_file(retrieve_file)
        .with_id_generator(id_generator)
        .with_resource_storage(resource_storage)
        .with_signature_storage(signature_storage)
        .with_upload_file(upload_file_storage)
        .with_file_metadata(file_metadata)
        // .with_authentication_gateway(LocalACLStore::new())
        // .with_authentication_strategy(JWTAuthentication::new());
        .build();

    match builder_result {
        Ok(app) => { app.run() }
        Err(_) => { }
    }

}