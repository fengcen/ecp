
use prelude::*;
use utils;


/// Struct builder.
#[derive(Clone)]
pub struct StructBuilder<'a, 'b: 'a> {
    cx: &'a ExtCtxt<'b>,
    ident: Option<Ident>,
    id: NodeId,
    span: Span,
    vis: Visibility,
    attrs: Vec<Attribute>,
    node: Option<ItemKind>,
}

impl<'a, 'b> StructBuilder<'a, 'b> {
    /// Create a struct builder.
    pub fn new(cx: &'a ExtCtxt<'b>) -> StructBuilder<'a, 'b> {
        StructBuilder {
            cx: cx,
            ident: None,
            id: DUMMY_NODE_ID,
            span: DUMMY_SP,
            vis: Visibility::Inherited,
            attrs: Vec::new(),
            node: None,
        }
    }

    /// Create from an `Item` and `ExtCtxt`.
    pub fn from(cx: &'a ExtCtxt<'b>, item: &Item) -> StructBuilder<'a, 'b> {
        StructBuilder {
            cx: cx,
            ident: Some(item.ident.clone()),
            id: item.id.clone(),
            span: item.span.clone(),
            vis: item.vis.clone(),
            attrs: item.attrs.clone(),
            node: Some(item.node.clone()),
        }
    }

    /// Add a `Derive` attribute.
    pub fn derive<T: AsRef<str>>(&mut self, attr: T) {
        let attr_ident = utils::to_ident(attr.as_ref());
        let attr = quote_attr!(self.cx, #[derive($attr_ident)]);
        self.attrs.push(attr);
    }

    /// Add an attribute.
    pub fn attr<T: AsRef<str>>(&mut self, attr: T) {
        let attr_ident = utils::to_ident(attr.as_ref());
        let attr = quote_attr!(self.cx, #[$attr_ident]);
        self.attrs.push(attr);
    }

    /// Get all attributes.
    pub fn get_attrs(&self) -> Vec<Attribute> {
        self.attrs.clone()
    }

    /// Get struct name.
    pub fn get_name(&self) -> Ident {
        self.ident.clone().unwrap()
    }

    /// Set struct name.
    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.ident = Some(utils::to_ident(name));
    }

    /// Get all struct fields.
    pub fn get_fields(&self) -> Vec<StructField> {
        if let Some(ItemKind::Struct(ref variant_data, _)) = self.node {
            if let VariantData::Struct(ref fields, _) = *variant_data {
                fields.clone()
            } else {
                panic!("not struct variant data");
            }
        } else {
            panic!("not struct item");
        }
    }

    /// Build an `Item`.
    pub fn build(self) -> Item {
        Item {
            ident: self.ident.unwrap(),
            id: self.id,
            span: self.span,
            vis: self.vis,
            attrs: self.attrs,
            node: self.node.unwrap(),
        }
    }
}
