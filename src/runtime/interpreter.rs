use super::environment::Environment;
use super::eval::evaluate_program;
use super::{NumberVal, RuntimeValue, ValueType};
use crate::parser::ast::NodeType;
use crate::runtime::eval::{
    eval_assignment, eval_identifier, eval_var_declaration, evaluate_binary_expression,
};

pub fn evaluate(ast_node: NodeType, env: &mut Environment) -> RuntimeValue {
    match ast_node {
        NodeType::NumericLiteral { value } => {
            RuntimeValue::NumberVal(NumberVal::new(ValueType::Number, value))
        }

        NodeType::BinaryExpr(binary_expr) => {
            return evaluate_binary_expression(binary_expr, env);
        }

        NodeType::Identifier(ast_node) => {
            return eval_identifier(ast_node, env);
        }

        NodeType::Program(program) => {
            return evaluate_program(program, env);
        }

        NodeType::VarDeclaration(var_declaration) => {
            return eval_var_declaration(var_declaration, env);
        }

        NodeType::AssignmentExpr(assignment_expr) => {
            return eval_assignment(assignment_expr, env);
        }
        #[allow(unused)]
        _ => panic!(
            "This AST Node has not been setup for the interpretation yet \n The node is: {:#?}",
            ast_node
        ),
    }
}
