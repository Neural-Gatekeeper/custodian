use std::io::{Error, ErrorKind};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Embedding,
    File,
    Model,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Embedding => "embedding",
            ResourceType::File => "file",
            ResourceType::Model => "model",
        }
    }
    pub fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewResourceRequest {
    pub resource_type: ResourceType,
}

impl NewResourceRequest {
    pub fn new(resource_type: &str) -> Result<NewResourceRequest, Error> {
        match resource_type {
            "embedding" => Ok(NewResourceRequest { resource_type: ResourceType::Embedding }),
            "file" => Ok(NewResourceRequest { resource_type: ResourceType::File }),
            "model" => Ok(NewResourceRequest { resource_type: ResourceType::Model }),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unknown resource type: {}", resource_type)))
        }
    }
}
