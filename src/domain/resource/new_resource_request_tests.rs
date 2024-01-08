use std::io::ErrorKind;
use crate::domain::resource::new_resource_request::NewResourceRequest;

#[test]
fn new_resource_request_embedding() {
    let result = NewResourceRequest::new("embedding");
    match result {
        Ok(request) => {
            match request.resource_type {
                crate::domain::resource::new_resource_request::ResourceType::Embedding => assert!(true),
                _ => assert!(false, "Should have been an embedding")
            }
        },
        Err(_) => assert!(false, "Should not have failed")
    }
}

#[test]
fn new_resource_request_file() {
    let result = NewResourceRequest::new("file");
    match result {
        Ok(request) => {
            match request.resource_type {
                crate::domain::resource::new_resource_request::ResourceType::File => assert!(true),
                _ => assert!(false, "Should have been a file")
            }
        },
        Err(_) => assert!(false, "Should not have failed")
    }
}

#[test]
fn new_resource_request_model() {
    let result = NewResourceRequest::new("model");
    match result {
        Ok(request) => {
            match request.resource_type {
                crate::domain::resource::new_resource_request::ResourceType::Model => assert!(true),
                _ => assert!(false, "Should have been a model")
            }
        },
        Err(_) => assert!(false, "Should not have failed")
    }
}

#[test]
fn new_resource_request_unknown() {
    let result = NewResourceRequest::new("unknown");
    match result {
        Ok(_) => assert!(false, "Should have failed"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput)
    }
}