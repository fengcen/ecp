
use prelude::*;

/// Attribute builder.
pub struct AttributeBuilder {
    raw: Option<Attribute>,
}

impl AttributeBuilder {
    /// Create a attribute builder.
    pub fn new() -> AttributeBuilder {
        AttributeBuilder { raw: None }
    }

    /// Get attribute name.
    pub fn get_name(&self) -> String {
        self.raw.as_ref().unwrap().value.name.to_string()
    }
}

impl From<Attribute> for AttributeBuilder {
    fn from(attribute: Attribute) -> AttributeBuilder {
        AttributeBuilder { raw: Some(attribute) }
    }
}
