#[derive(Debug)]
pub enum NodeType {
    //Statments
    Program(Program),
    VarDeclaration(VarDeclaration),
    // Expressions
    NumericLiteral { value: f64 },
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
}

#[derive(Debug)]
pub struct VarDeclaration {
    is_constant: bool,
    identifier: String,
    value: Option<Box<NodeType>>, // this field is an option to handle nonasignment declaration like lex x;
}

impl VarDeclaration {
    pub fn new(identifier: String, is_constant: bool, value: Option<Box<NodeType>>) -> Self {
        Self {
            is_constant,
            identifier,
            value,
        }
    }
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
