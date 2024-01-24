use std::io::Error;
use crate::kernel::values::value_object::ValueObject;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Result<Id, Error> {
        if id > 0 {
            Ok(Id(id))
        } else {
            Err(Error::new(std::io::ErrorKind::InvalidInput,
                           format!("The id must be greater than 0, but was {}", id)))
        }
    }
}

impl ValueObject<u64> for Id {
    fn value(&self) -> u64 {
        self.0
    }
}

impl PartialEq<u64> for Id {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Id> for u64 {
    fn eq(&self, other: &Id) -> bool {
        *self == other.0
    }
}
