
use prelude::*;

/// Convert an Item to Annotatable.
pub fn to_annotatable(item: Item) -> Annotatable {
    Annotatable::Item(P(item))
}

/// Convert a string to Name.
pub fn to_name<T: AsRef<str>>(name: T) -> Name {
    Symbol::intern(name.as_ref())
}

/// Convert a string to Ident.
pub fn to_ident<T: AsRef<str>>(ident: T) -> Ident {
    Ident::with_empty_ctxt(to_name(ident.as_ref()))
}

/// Generic convert to Spanned.
pub fn to_spanned<T>(t: T) -> Spanned<T> {
    Spanned {
        node: t,
        span: DUMMY_SP,
    }
}

/// Convert an expression to a statement.
pub fn expr_to_stmt(expr: Expr) -> Stmt {
    Stmt {
        id: DUMMY_NODE_ID,
        node: StmtKind::Expr(P(expr)),
        span: DUMMY_SP,
    }
}

/// Convert an expression which contains semicolon to a statement.
pub fn semi_to_stmt(expr: Expr) -> Stmt {
    Stmt {
        id: DUMMY_NODE_ID,
        node: StmtKind::Semi(P(expr)),
        span: DUMMY_SP,
    }
}
