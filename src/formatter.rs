use std::fmt::Write;

use crate::expression::Expression;

pub fn colorize_string(color: &str, str: &str) -> String {
    match color {
        "red" => format!(r#"\u001b[31m{str}\u001b[0m"#),
        "green" => format!(r#"\u001b[32m{str}\u001b[0m"#),
        "yellow" => format!(r#"\u001b[33m{str}\u001b[0m"#),
        "blue" => format!(r#"\u001b[34m{str}\u001b[0m"#),
        "magenta" => format!(r#"\u001b[35m{str}\u001b[0m"#),
        "cyan" => format!(r#"\u001b[36m{str}\u001b[0m"#),
        _ => str.into(),
    }
}

pub fn get_args_as_string(args: &[Expression]) -> String {
    let mut arguments_string = String::new();

    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        arguments_string
            .write_str(&match arg.clone() {
                Expression::FnCall(fn_call) => match fn_call.name.lexeme.as_str() {
                    "red" | "green" | "yellow" | "blue" | "magenta" | "cyan" => {
                        format!("\"{}\"", arg.write())
                    }
                    _ => format!("\"$({})\"", arg.write()),
                },
                _ => arg.write(),
            })
            .unwrap();

        if iter.peek().is_some() {
            arguments_string.write_str(" ").unwrap();
        }
    }

    arguments_string
}

pub fn write_formatted_expressions(expressions: &[Expression]) -> String {
    let mut output = String::new();

    for expr in expressions {
        let lines: Vec<String> = expr
            .write()
            .split('\n')
            .map(|line| format!("    {line}\n"))
            .collect();

        let lines = lines.join("");

        output.write_str(&lines).unwrap();
    }

    output
}
