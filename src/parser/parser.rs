//parser.rs

use super::Program;
use crate::{
    lexer::{Token, TokenType, tokenize},
    parser::{
        Identifier,
        ast::{BinaryExpr, NodeType},
    },
};
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn produce_ast(&mut self, source_code: &str) -> NodeType {
        self.tokens = tokenize(source_code);
        let mut nodes: Vec<NodeType> = Vec::new();
        while !self.is_eof() {
            nodes.push(self.parse_expr());
        }
        let program: NodeType = NodeType::Program(Program { body: nodes });
        return program;
    }

    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            pos: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.tokens[self.pos].token_type == TokenType::EOF
    }

    fn eat(&mut self) -> Option<Token> {
        if !self.is_eof() {
            let token: Token = self.tokens[self.pos].clone();
            self.pos += 1;
            return Some(token);
        } else {
            None
        }
    }

    fn at(&self) -> &Token {
        &self.tokens[self.pos]
    }

    // fn peek_back(&self) -> Option<&Token> {
    //     if self.pos > 0 {
    //         Some(&self.tokens[self.pos - 1])
    //     } else {
    //         None
    //     }
    // }

    // Order of Prescidence 👇
    // AssignmentExpr
    // MemberExpr
    // FunctionCall
    // LogicalExpr
    // ComparisonExpr
    // AdditiveExpr
    // MultiplicitaveExpt
    // UnaryExpr
    // PrimaryExpr
    fn parse_expr(&mut self) -> NodeType {
        return self.parse_additive();
    }

    // Example: 10 + 5 - 5 =should be>
    /**
     * {
     *      left: {
     *          left: 10,
     *          right: 5,
     *          operator: +
     *      },
     *      right: 5,
     *      operator: -
     * }
     */
    fn parse_additive(&mut self) -> NodeType {
        let mut left = self.parse_multiplicitave();
        while self.at().token_type == TokenType::AdittiveOperator {
            if let Some(token) = self.eat() {
                let right = self.parse_multiplicitave();
                left = NodeType::BinaryExpr(BinaryExpr {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: token.value,
                })
            }
        }

        return left;
    }

    fn parse_multiplicitave(&mut self) -> NodeType {
        let mut left = self.parse_primary_expr();
        while self.at().token_type == TokenType::MultiplicitaveOperator {
            if let Some(token) = self.eat() {
                let right = self.parse_primary_expr();
                left = NodeType::BinaryExpr(BinaryExpr {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: token.value,
                })
            }
        }

        return left;
    }

    fn parse_primary_expr(&mut self) -> NodeType {
        if let Some(token) = self.eat() {
            match token.token_type {
                TokenType::Identifier => NodeType::Identifier(Identifier {
                    symbol: token.value,
                }),
                TokenType::Number => match token.value.parse::<f64>() {
                    Ok(value) => NodeType::NumericLiteral { value },
                    Err(_) => panic!("Invalid number literal"),
                },

                TokenType::Null => NodeType::NullLiteral,

                TokenType::OpenParen => {
                    println!("OpenParen");
                    let value: NodeType = self.parse_expr();
                    if let Some(e) = self.eat() {
                        if e.token_type != TokenType::CloseParen {
                            panic!("Expected ), but got {:#?}", e)
                        }
                    } else {
                        panic!("Expected ) at the end of the expression")
                    }
                    value
                }
                TokenType::CloseParen => {
                    panic!("Closing PAREN")
                }

                TokenType::EOF => NodeType::Identifier(Identifier {
                    symbol: String::from("EOF"),
                }),
                _ => panic!("There is something wrong {:#?}", token),
            }
        } else {
            NodeType::Identifier(Identifier {
                symbol: String::from("SOMETHING ELSE"),
            })
        }
    }
}
