use crate::{
    parser::{
        AssignmentExpr, Identifier,
        ast::{BinaryExpr, NodeType},
    },
    runtime::{Environment, NullVal, NumberVal, RuntimeValue, evaluate},
};

pub fn evaluate_binary_expression(binary_expr: BinaryExpr, env: &mut Environment) -> RuntimeValue {
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

pub fn eval_identifier(idt: Identifier, env: &mut Environment) -> RuntimeValue {
    let val = env.lookup_var(&idt.symbol);
    val.clone()
}

pub fn eval_assignment(node: AssignmentExpr, env: &mut Environment) -> RuntimeValue {
    let assignee = *node.assignee;
    if !matches!(assignee, NodeType::Identifier(_)) {}

    match assignee {
        NodeType::Identifier(identifier) => {
            let var_name = identifier.symbol;
            let value = evaluate(*node.value, env);
            env.assign_variable(&var_name, value)
        }

        _ => panic!("Invalid LHS inaide assignment expression {:#?}", assignee),
    }
}
