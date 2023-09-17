use cel_interpreter::{Context, Program, Value};

fn main() {
    let expressions = [
        "b && (c == \"string\")",
        "b && c == \"string\"",  // This expression is incorrectly evaluated
        "c == \"string\" && b",
    ];

    for expression in expressions {
        let program = Program::compile(expression).unwrap();
        let mut context = Context::default();
        context.add_variable("b", Value::Bool(true));
        context.add_variable("c", Value::String(String::from("string").into()));

        let result = program.execute(&context);

        println!("{:?} <= {}", result, expression)
    }
}