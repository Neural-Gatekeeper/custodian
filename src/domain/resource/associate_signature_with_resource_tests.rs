use std::io::{Error, ErrorKind};
use std::sync::Arc;
use crate::domain::resource::associate_signature_with_resource::{associate_signature_with_resource, AssociateSignatureRequest};
use crate::domain::resource::gateway::signature_storage::SignatureStorage;
use crate::kernel::values::ResourceSignature;
use crate::kernel::values::resource_id::ResourceId;

#[test]
fn successful_assign_signature_to_resource() {

    let request = AssociateSignatureRequest { id: ResourceId::new(1).unwrap(), signature: String::from("some-signature") };
    let storage = Arc:: new(MockSignatureStorage{}) as Arc<dyn SignatureStorage>;
        match associate_signature_with_resource(request, storage.as_ref()) {
        Ok(_) => { assert!(true); }
        Err(_) => {  assert!(false, "Should not have failed"); }
    }
}
#[test]
fn signature_store_returns_error() {
    let request = AssociateSignatureRequest { id: ResourceId::new(1).unwrap(), signature: String::from("some-signature") };
    let storage = Arc::new(ErrorSignatureStorage {}) as Arc<dyn SignatureStorage>;
    match associate_signature_with_resource(request, storage.as_ref()) {
        Ok(_) => { assert!(false, "Should not have failed"); }
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::Other);
            assert_eq!(e.to_string(), "Could not assign the signature [some-signature] to the resource id [1]: Error from ErrorSignatureStorage");
        }
    }
}


struct MockSignatureStorage { }
impl SignatureStorage for MockSignatureStorage {
    fn associate(&self, _id: ResourceId, _signature: ResourceSignature) -> Result<(), Error> {
        Ok(())
    }

}
    struct ErrorSignatureStorage { }
    impl SignatureStorage for ErrorSignatureStorage {
        fn associate(&self, _id: ResourceId, _signature: ResourceSignature) -> Result<(), Error> {
            Err(Error::new(ErrorKind::Other, "Error from ErrorSignatureStorage"))
        }
    }