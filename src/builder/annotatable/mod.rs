
use prelude::*;

/// Annotatable builder.
#[derive(Debug, Clone)]
pub struct AnnotatableBuilder {
    node: Option<Annotatable>,
}

impl AnnotatableBuilder {
    /// Create a annotatable builder.
    pub fn new() -> AnnotatableBuilder {
        AnnotatableBuilder { node: None }
    }

    /// Get item from `Annotatable`.
    pub fn get_item(&self) -> Item {
        if let Annotatable::Item(ref item_p) = *self.node.as_ref().unwrap() {
            item_p.clone().unwrap()
        } else {
            panic!("not annotatable item");
        }
    }
}

impl<'a> From<&'a Annotatable> for AnnotatableBuilder {
    fn from(annotatable: &Annotatable) -> AnnotatableBuilder {
        AnnotatableBuilder { node: Some(annotatable.clone()) }
    }
}
