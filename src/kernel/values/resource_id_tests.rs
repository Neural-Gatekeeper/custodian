use crate::kernel::values::resource_id::ResourceId;
use std::io::ErrorKind;
use crate::kernel::values::value_object::ValueObject;

#[test]
fn should_create_a_resource_id() {
    let resource_id = ResourceId::new(1);
    assert!(resource_id.is_ok());
    assert!(resource_id.as_ref().unwrap().equals(&ResourceId::new(1).unwrap()));
    assert!(resource_id.as_ref().unwrap().equals_value(1));
}

#[test]
fn should_not_create_a_resource_id_with_zero() {
    let resource_id = ResourceId::new(0);
    assert!(resource_id.is_err());
    assert_eq!(resource_id.unwrap_err().kind(), ErrorKind::InvalidInput);
}

#[test]
fn should_return_the_value_of_the_resource_id() {
    let resource_id = ResourceId::new(1).unwrap();
    assert_eq!(resource_id.value(), 1);
}