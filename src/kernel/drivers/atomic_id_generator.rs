use std::io::{Error};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;
use crate::kernel::values::id::Id;

pub struct AtomicIdGenerator {
    counter: AtomicU64,
}

impl AtomicIdGenerator {
    pub fn new() -> Self {
        AtomicIdGenerator {
            counter: AtomicU64::new(1),
        }
    }
}
impl IdGenerator for AtomicIdGenerator {
    fn generate(&self) -> Result<Id, Error> {
        let id =self.counter.fetch_add(1, Ordering::SeqCst);
        Id::new(id)
    }
}
