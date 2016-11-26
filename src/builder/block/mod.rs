
use prelude::*;

/// Block builder.
#[derive(Debug, Clone)]
pub struct BlockBuilder {
    stmts: Vec<Stmt>,
    id: NodeId,
    rules: BlockCheckMode,
    span: Span,
}

impl BlockBuilder {
    /// Create a block builder.
    pub fn new() -> BlockBuilder {
        BlockBuilder {
            stmts: Vec::new(),
            id: DUMMY_NODE_ID,
            rules: BlockCheckMode::Default,
            span: DUMMY_SP,
        }
    }

    /// Add a statement.
    pub fn stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    /// Build a `Block`.
    pub fn build(self) -> Block {
        Block {
            stmts: self.stmts.clone(),
            id: self.id.clone(),
            rules: self.rules.clone(),
            span: self.span.clone(),
        }
    }
}
