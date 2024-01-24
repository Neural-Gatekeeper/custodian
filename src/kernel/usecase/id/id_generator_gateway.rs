use std::io::Error;
use crate::kernel::values::id::Id;

pub trait IdGenerator: Sync + Send {
    fn generate(&self) -> Result<Id, Error>;

}