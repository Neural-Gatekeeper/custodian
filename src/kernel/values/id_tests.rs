use crate::kernel::values::id::Id;
use std::io::ErrorKind;
use crate::kernel::values::value_object::ValueObject;

#[test]
fn should_create_a_resource_id() {
    let resource_id = Id::new(1);
    assert!(resource_id.is_ok());
    assert!(resource_id.as_ref().unwrap().equals(&Id::new(1).unwrap()));
    assert!(resource_id.as_ref().unwrap().equals_value(1));
}

#[test]
fn should_not_create_a_resource_id_with_zero() {
    let resource_id = Id::new(0);
    assert!(resource_id.is_err());
    assert_eq!(resource_id.as_ref().unwrap_err().kind(), ErrorKind::InvalidInput);
    assert_eq!(resource_id.as_ref().unwrap_err().to_string(), "The id must be greater than 0, but was 0");
}

#[test]
fn should_return_the_value_of_the_resource_id() {
    let resource_id = Id::new(1).unwrap();
    assert_eq!(resource_id.value(), 1);
}