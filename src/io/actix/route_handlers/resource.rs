use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use crate::domain::resource::generate_new_resource::generate_new_resource;
use crate::domain::resource::new_resource_request::{NewResourceRequest};
use crate::domain::resource::gateway::resource_storage::ResourceStorage;
use crate::kernel::gateways::resource_generator::ResourceGenerator;
use crate::kernel::values::resource_id::ResourceId;
use crate::kernel::values::value_object::ValueObject;

#[derive(Deserialize)]
pub struct NewResourceHttpRequest {
    pub resource_type: String,
}
#[derive(Serialize)]
pub struct NewResourceHttpResponse {
    pub id: u64,
    pub resource_type: String,
}

impl NewResourceHttpResponse {
    pub fn new(id: ResourceId, request: NewResourceRequest) -> Self {
        Self { id: id.value(), resource_type: request.resource_type.to_string() }
    }
}

pub async fn post(new_request: Json<NewResourceHttpRequest>,
                  generator: Data<Arc<dyn ResourceGenerator>>,
                  store: Data<Arc<dyn ResourceStorage>>) -> impl Responder {

    let new_request = match NewResourceRequest::new(new_request.resource_type.as_str()) {
        Ok(new_request) => new_request,
        Err(e) => { return HttpResponse::BadRequest().body(e.to_string()) }
    };

    match generate_new_resource(new_request.clone(), generator.as_ref().as_ref(), store.as_ref().as_ref()) {
        Ok(id) => HttpResponse::Ok().json(NewResourceHttpResponse::new(id, new_request)),
        Err(_) => HttpResponse::InternalServerError().body("We were unable to generate a resource id for you."),
    }
}