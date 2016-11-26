
use prelude::*;

/// MetaItem builder.
pub struct MetaItemBuilder {
    node: Option<MetaItem>,
}

impl MetaItemBuilder {
    /// Create a metaitem builder.
    pub fn new() -> MetaItemBuilder {
        MetaItemBuilder { node: None }
    }

    /// Get meta words.
    pub fn get_words(&self) -> Vec<String> {
        let mut values = Vec::new();

        if let MetaItemKind::List(ref inners) = self.node.as_ref().unwrap().node {
            for inner in inners {
                if let NestedMetaItemKind::MetaItem(ref inner) = inner.node {
                    if let MetaItemKind::Word = inner.node {
                        values.push(inner.name.to_string());
                    }
                }
            }
        }

        values
    }

    /// Get `NameValue`'s value.
    pub fn get_name_value(&self) -> Option<String> {
        if let MetaItemKind::NameValue(ref spanned) = self.node.as_ref().unwrap().node {
            if let LitKind::Str(ref value, _) = spanned.node {
                return Some(value.to_string());
            }
        }

        None
    }

    /// Get meta name.
    pub fn get_name(&self) -> String {
        self.node.as_ref().unwrap().name.to_string()
    }
}

impl<'a> From<&'a MetaItem> for MetaItemBuilder {
    fn from(meta: &MetaItem) -> MetaItemBuilder {
        MetaItemBuilder { node: Some(meta.clone()) }
    }
}
