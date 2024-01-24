use std::io::{Error, ErrorKind};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::id::Id;
use crate::kernel::values::Signature;
use crate::kernel::values::value_object::ValueObject;

#[derive(Debug)]
pub struct AssociateSignatureRequest {
    pub id: Id,
    pub signature: Signature,
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