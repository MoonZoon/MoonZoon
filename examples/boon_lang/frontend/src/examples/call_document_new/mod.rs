use crate::boon::platform::browser::preludes::for_generated_code::{println, eprintln, *};

use crate::boon::parser::{parser, Parser};

#[allow(dead_code)]
pub async fn run() -> Arc<Object> {
    let program = include_str!("call_document_new.bn");
    println!("{program}");

    let ast = parser().parse(program).into_result();
    println!("{:#?}", ast);
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
            panic!("Failed to parse the Boon program, see the errors above.")
        }
    }
}
