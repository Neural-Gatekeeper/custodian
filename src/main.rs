use std::sync::Arc;
use crate::application::custodian;
use domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::file::gateway::file_storage::FileStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::io::actix::actix_web_server::ActixWebServerBuilder;
use crate::io::std::std_fs_filestore::StdFSFileStorage;
use crate::kernel::drivers::atomic_resource_generator::AtomicResourceGenerator;
use crate::kernel::drivers::inmemory_resource_storage::InMemoryResourceStorage;
use crate::kernel::gateways::resource_generator::ResourceGenerator;

mod application;
mod domain;
mod io;
mod kernel;

fn main() {

    // todo - LEFT OFF - implement #3 and create an file-storage gateway for #3
    // 1) upload file, 2) create a resoruce id, 3) get the sha of the data, 4) associate the sha with the resource id
    // todo - 3 - create a usecase to associate a file/ file-signature with a with a resource-id.
        // update the /file to be a POST and provide the resource-id as a form-data param.
    // todo - 4 - create a usecase to upload a file to the filetorage
        // when you upload a file, give it a resource id as well as take the signature, and associate the signature with the resrouce id
    // todo - 5 create a usecase to get a represenation of all the files associated with a specific resoruce

    let std_fs_file_storage= Arc::new(StdFSFileStorage::new());
    let storage_gateway = Arc::clone(&std_fs_file_storage) as Arc<dyn FileStorage>;

    let atomic_resource_generator = Arc::new(AtomicResourceGenerator::new());
    let resource_generator = Arc::clone(&atomic_resource_generator) as Arc<dyn ResourceGenerator>;

    let inmemory_resource_storage = Arc::new(InMemoryResourceStorage::new());
    let resource_storage = Arc::clone(&inmemory_resource_storage) as Arc<dyn ResourceStorage>;
    let signature_storage = Arc::clone(&inmemory_resource_storage) as Arc<dyn SignatureStorage>;

    let builder_result = custodian::CustodianAppBuilder::new()
        .using_io_channel(ActixWebServerBuilder::new("127.0.0.1", 8000))
        .with_file_storage(storage_gateway)
        .with_resource_generator(resource_generator)
        .with_resource_storage(resource_storage)
        .with_signature_storage(signature_storage)
        // .with_authentication_gateway(LocalACLStore::new())
        // .with_authentication_strategy(JWTAuthentication::new());
        .build();

    match builder_result {
        Ok(app) => { app.run() }
        Err(_) => { }
    }

}