use std::io::Error;
use crate::kernel::values::value_object::ValueObject;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceId(u64);

impl ResourceId {
    pub fn new(id: u64) -> Result<ResourceId, Error> {
        if id > 0 {
            Ok(ResourceId(id))
        } else {
            Err(Error::new(std::io::ErrorKind::InvalidInput,
                           format!("The resource id must be greater than 0, but was {}", id)))
        }
    }
}

impl ValueObject<u64> for ResourceId {
    fn value(&self) -> u64 {
        self.0
    }
}

impl PartialEq<u64> for ResourceId {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ResourceId> for u64 {
    fn eq(&self, other: &ResourceId) -> bool {
        *self == other.0
    }
}
