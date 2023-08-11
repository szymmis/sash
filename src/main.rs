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

    match args.output_filename {
        Some(path) => code.write_file(&path),
        None => {
            code.debug();
            print!("------ generated code ------\n{}", code.get_code());
            println!("--------- output ----------");
            code.interpret();
        }
    }
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let input_filename = args
        .get(1)
        .expect("You need to pass filename as an argument!")
        .to_owned();

    let output_filename = args.get(2).cloned();

    Args {
        input_filename,
        output_filename,
    }
}
