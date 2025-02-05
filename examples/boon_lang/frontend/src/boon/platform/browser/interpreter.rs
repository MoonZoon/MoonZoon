use crate::boon::platform::browser::preludes::for_generated_code::{println, eprintln, *};

use crate::boon::parser::{parser, Parser};
use crate::boon::lexer::lexer;

pub fn run(source_code: &str) -> Arc<Object> {
    println!("[Boon source code]");
    println!("{source_code}");

    let tokens = lexer().parse(source_code).into_result();
    println!("[Tokens]");
    println!("{tokens:#?}");
    
    let ast = parser().parse(source_code).into_result();
    println!("[AST]");
    println!("{ast:#?}");
    match ast {
        Ok(ast) => match evaluate(&ast) {
            Ok(output) => output,
            Err(evaluation_error) => {
                panic!("Evaluation error: {evaluation_error}");
            }
        }
        Err(parse_errors) => {
            for parse_error in parse_errors {
                eprintln!("Parse error: {parse_error}");
            }
            panic!("Failed to parse the Boon source code, see the errors above.")
        }
    }
}
