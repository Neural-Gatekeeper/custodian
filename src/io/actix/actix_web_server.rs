use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use crate::domain::file::gateway::file_storage::FileStorage;
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::input_source::{InputSource, InputSourceBuilder};
use crate::io::actix::route_handlers::{file, resource, resource_signature};
use crate::kernel::gateways::resource_generator::ResourceGenerator;

pub struct ActixWebServer {
    host: String,
    port: u16,
    file_storage: Arc<dyn FileStorage>,
    resource_generator: Arc<dyn ResourceGenerator>,
    resource_storage: Arc<dyn ResourceStorage>,
    signature_storage: Arc<dyn SignatureStorage>
}
impl InputSource for ActixWebServer {
    fn run(&self) {
        let file_storage = Arc::clone(&self.file_storage);
        let resource_generator = Arc::clone(&self.resource_generator);
        let resource_storage = Arc::clone(&self.resource_storage);
        let signature_storage = Arc::clone(&self.signature_storage);
        let host = self.host.clone();
        let port = self.port.clone();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let _ = HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(Arc::clone(&file_storage)))
                        .app_data(Data::new(Arc::clone(&resource_generator)))
                        .app_data(Data::new(Arc::clone(&resource_storage)))
                        .app_data(Data::new(Arc::clone(&signature_storage)))
                        .route("/file", web::get().to(file::get))
                        .route("/resource", web::post().to(resource::post))
                        .route("/resource/signature", web::post().to(resource_signature::post))
                })
                    .bind(format!("{}:{}", host, port))
                    .unwrap()
                    .run()
                    .await;
            })
    }

}

#[derive(Clone)]
pub struct ActixWebServerBuilder {
    host: String,
    port: u16,
    file_storage: Option<Arc<dyn FileStorage>>,
    resource_generator: Option<Arc<dyn ResourceGenerator>>,
    resource_storage: Option<Arc<dyn ResourceStorage>>,
    signature_storage: Option<Arc<dyn SignatureStorage>>,
}
impl ActixWebServerBuilder {
    pub(crate) fn new(host: &str, port: u16) -> Box<dyn InputSourceBuilder> {
        Box::new(ActixWebServerBuilder {
            host: host.to_string(),
            port,
            file_storage: None,
            resource_generator: None,
            resource_storage: None,
            signature_storage: None,
        })
    }
}
impl InputSourceBuilder for ActixWebServerBuilder {

    fn with_storage_gateway(&mut self, gateway: Arc<dyn FileStorage>) -> Box<dyn InputSourceBuilder> {
        self.file_storage = Some(Arc::clone(&gateway));
        Box::new(self.clone())
    }

    fn with_resource_generator(&mut self, generator: Arc<dyn ResourceGenerator>) -> Box<dyn InputSourceBuilder> {
        self.resource_generator = Some(Arc::clone(&generator));
        Box::new(self.clone())
    }

    fn with_resource_storage(&mut self, storage: Arc<dyn ResourceStorage>) -> Box<dyn InputSourceBuilder> {
        self.resource_storage = Some(Arc::clone(&storage));
        Box::new(self.clone())
    }

    fn with_signature_storage(&mut self, storage: Arc<dyn SignatureStorage>) -> Box<dyn InputSourceBuilder> {
        self.signature_storage = Some(Arc::clone(&storage));
        Box::new(self.clone())
    }

    fn build(&self) -> Box<dyn InputSource> {
        Box::new(ActixWebServer {
            host: self.host.clone(),
            port: self.port.clone(),
            file_storage: Arc::clone(&self.file_storage.as_ref().unwrap()),
            resource_generator: Arc::clone(&self.resource_generator.as_ref().unwrap()),
            resource_storage: Arc::clone(&self.resource_storage.as_ref().unwrap()),
            signature_storage: Arc::clone(&self.signature_storage.as_ref().unwrap()),
        })
    }

}

