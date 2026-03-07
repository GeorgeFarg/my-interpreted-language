use super::{NullVal, NumberVal, RuntimeValue, ValueType};

use super::environment::Environment;
use crate::parser::ast::{BinaryExpr, NodeType};
use crate::parser::{Identifier, Program};

pub fn evaluate(ast_node: NodeType, env: &mut Environment) -> RuntimeValue {
    match ast_node {
        NodeType::NumericLiteral { value } => {
            RuntimeValue::NumberVal(NumberVal::new(ValueType::Number, value))
        }
        NodeType::NullLiteral => RuntimeValue::NullVal(NullVal::new()),

        NodeType::BinaryExpr(binary_expr) => {
            return evaluate_binary_expression(binary_expr, env);
        }

        NodeType::Identifier(ast_node) => {
            return eval_identifier(ast_node, env);
        }

        NodeType::Program(program) => {
            return evaluate_program(program, env);
        }
        _ => panic!(
            "This AST Node has not been setup for the interpretation yet \n The node is: {:#?}",
            ast_node
        ),
    }
}

fn evaluate_binary_expression(binary_expr: BinaryExpr, env: &mut Environment) -> RuntimeValue {
    let left_hand_side: RuntimeValue = evaluate(*binary_expr.left, env);
    let right_hand_side: RuntimeValue = evaluate(*binary_expr.right, env);

    match (left_hand_side, right_hand_side) {
        (RuntimeValue::NumberVal(lhs), RuntimeValue::NumberVal(rhs)) => {
            let evaluated_binary_expression =
                evaluate_numeric_binary_expression(lhs, rhs, &binary_expr.operator);
            RuntimeValue::NumberVal(evaluated_binary_expression)
        }

        _ => RuntimeValue::NullVal(NullVal::new()),
    }
}

fn evaluate_numeric_binary_expression(lhs: NumberVal, rhs: NumberVal, operator: &str) -> NumberVal {
    match operator {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => {
            "Division vy zero checks";
            lhs / rhs
        }
        "%" => lhs % rhs,
        _ => {
            panic!("An unrecognised operator")
        }
    }
}

fn eval_identifier(idt: Identifier, env: &mut Environment) -> RuntimeValue {
    let val = env.lookup_var(&idt.symbol);
    val.clone()
}

fn evaluate_program(program: Program, env: &mut Environment) -> RuntimeValue {
    let mut last_evaluated: RuntimeValue = RuntimeValue::NullVal(NullVal::new());

    for stmt in program.body {
        last_evaluated = evaluate(stmt, env);
    }

    return last_evaluated;
}
