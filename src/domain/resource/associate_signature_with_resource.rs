use std::io::{Error, ErrorKind};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::resource_id::ResourceId;
use crate::kernel::values::ResourceSignature;
use crate::kernel::values::value_object::ValueObject;

#[derive(Debug)]
pub struct AssociateSignatureRequest {
    pub id: ResourceId,
    pub signature: ResourceSignature,
}

pub fn associate_signature_with_resource(
    request: AssociateSignatureRequest,
    storage: &dyn SignatureStorage,
) -> Result<(), Error> {
    match storage.associate(request.id, request.signature.clone()) {
        Ok(_) => { Ok(()) }
        Err(e) => { Err(Error::new(ErrorKind::Other,
                                  format!("Could not assign the signature [{}] to the resource id [{}]: {}", request.signature, request.id.value(), e))) }
    }
}