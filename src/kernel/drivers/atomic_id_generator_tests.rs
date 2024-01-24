use std::sync::Arc;
use crate::kernel::drivers::atomic_id_generator::AtomicIdGenerator;
use crate::kernel::usecase::id::id_generator_gateway::IdGenerator;
use crate::kernel::values::value_object::ValueObject;

#[test]
fn test_generate_id() {
    let generator: Arc<dyn IdGenerator> = Arc::new(AtomicIdGenerator::new());
    match generator.generate() {
        Ok(resource_id) => { assert!(resource_id.value() > 0); }
        Err(e) => {
            assert!(false, "{}", format!("Received and error when no error was expected: {}", e));
        }
    }
}

#[test]
fn subsequent_ids_greater_than_previous() {
    let generator: Arc<dyn IdGenerator> = Arc::new(AtomicIdGenerator::new());
    let id1 = match generator.generate() {
        Ok(resource_id) => { resource_id }
        Err(e) => {
            panic!("Received and error when no error was expected: {}", e);
        }
    };
    let id2 = match generator.generate() {
        Ok(resource_id) => { resource_id }
        Err(e) => {
            panic!("Received and error when no error was expected: {}", e);
        }
    };
    let id3 = match generator.generate() {
        Ok(resource_id) => { resource_id }
        Err(e) => {
            panic!("Received and error when no error was expected: {}", e);
        }
    };

    assert!(id1.value() > 0);
    assert!(id2.value() > 0);
    assert!(id3.value() > 0);
    assert!(id2.value() > id1.value());
    assert!(id3.value() > id2.value());
}