use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use crate::domain::resource::associate_signature_with_resource::{associate_signature_with_resource, AssociateSignatureRequest};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::id::Id;

#[derive(Deserialize)]
pub struct AssociateSignatureHTTPRequest {
    pub id: u64,
    pub signature: String,
}

pub async fn post(request: Json<AssociateSignatureHTTPRequest>,
                  signature_storage: Data<Arc<dyn SignatureStorage>>) -> impl Responder {

    let request = match Id::new(request.id) {
        Ok(id ) => { AssociateSignatureRequest { id, signature: request.signature.clone(), } }
        Err(e) => { return HttpResponse::BadRequest().body(e.to_string()) }
    };

    // todo - future - add a logger to capture the error from the use case that is not passed back to the user
    match associate_signature_with_resource(request, signature_storage.get_ref().as_ref()) {
        Ok(_) => HttpResponse::Created().finish() ,
        Err(_) => HttpResponse::InternalServerError()
            .body("We were unable to associate the signature with the resource."),
    }

}