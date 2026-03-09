use crate::{
    parser::{Program, ast::VarDeclaration},
    runtime::{Environment, NullVal, RuntimeValue, evaluate},
};

pub fn evaluate_program(program: Program, env: &mut Environment) -> RuntimeValue {
    let mut last_evaluated: RuntimeValue = RuntimeValue::NullVal(NullVal::new());

    for stmt in program.body {
        last_evaluated = evaluate(stmt, env);
    }

    return last_evaluated;
}

pub fn eval_var_declaration(
    var_declaration: VarDeclaration,
    env: &mut Environment,
) -> RuntimeValue {
    match var_declaration.value {
        Some(node) => {
            let value = evaluate(*node, env);
            env.declare_var(
                &var_declaration.identifier,
                value,
                var_declaration.is_constant,
            )
        }
        None => RuntimeValue::NullVal(NullVal::new()),
    }
}
