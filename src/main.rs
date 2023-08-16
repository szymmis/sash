#![warn(clippy::pedantic)]
extern crate sash_lang;
use sash_lang::Script;

use std::env;

struct Args {
    input_filename: String,
    output_filename: Option<String>,
}

fn main() {
    let args = parse_args();

    let code = Script::from_file(&args.input_filename);

    if let Some(path) = args.output_filename {
        code.write_file(&path);
    } else {
        print!("------ generated code ------\n{}", code.get_code());
        println!("--------- output ----------");
        code.interpret();
    }
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let input_filename = args
        .get(1)
        .expect("You need to pass filename as an argument!")
        .clone();

    let output_filename = args.get(2).cloned();

    Args {
        input_filename,
        output_filename,
    }
}
