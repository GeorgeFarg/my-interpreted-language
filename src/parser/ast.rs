#[derive(Debug)]
pub enum NodeType {
    Program(Program),
    NumericLiteral { value: f64 },
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
}

#[derive(Debug)]
pub struct Identifier {
    pub(crate) symbol: String,
}

#[derive(Debug)]
pub struct Program {
    pub(crate) body: Vec<NodeType>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub(crate) left: Box<NodeType>,
    pub(crate) right: Box<NodeType>,
    pub(crate) operator: String,
}

impl Program {
    pub fn new(body: Vec<NodeType>) -> Self {
        Program { body }
    }
}
