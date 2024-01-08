use std::io::{Error};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::kernel::gateways::resource_generator::ResourceGenerator;
use crate::kernel::values::resource_id::ResourceId;

pub struct AtomicResourceGenerator {
    counter: AtomicU64,
}

impl AtomicResourceGenerator {
    pub fn new() -> Self {
        AtomicResourceGenerator {
            counter: AtomicU64::new(1),
        }
    }
}
impl ResourceGenerator for AtomicResourceGenerator {
    fn generate_id(&self) -> Result<ResourceId, Error> {
        let id =self.counter.fetch_add(1, Ordering::SeqCst);
        ResourceId::new(id)
    }
}
