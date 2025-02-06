use std::io::{Cursor, Read};
use ariadne::{Source, Color, Label, Report, ReportKind, Config};
use chumsky::prelude::{Rich, SimpleSpan};

use crate::boon::platform::browser::preludes::for_generated_code::{println, eprintln, *};
use crate::boon::parser::{parser, Parser};
use crate::boon::lexer::lexer;

pub fn run(filename: &str, source_code: &str) -> Arc<Object> {
    println!("[Source Code ({filename})]");
    println!("{source_code}");

    let (tokens, errors) = lexer().parse(source_code).into_output_errors();
    println!("[Tokens]");
    println!("{tokens:?}");
    if !errors.is_empty() {
        println!("[Lex Errors]");
    }
    report_errors(errors, filename, source_code);
    
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

fn report_errors(errors: Vec<Rich<char, SimpleSpan>>, filename: &str, source_code: &str) {
    let mut report_bytes = Cursor::new(Vec::new());
    let mut report_string = String::new();
    for error in errors {
        report_bytes.set_position(0);
        report_bytes.get_mut().clear();
        Report::build(ReportKind::Error, (filename, error.span().into_range()))
            .with_config(Config::default().with_color(false))
            .with_message(error.to_string())
            .with_label(
                Label::new((filename, error.span().into_range()))
                    .with_message(error.reason().to_string())
            )
            .finish()
            .write((filename, Source::from(source_code)), &mut report_bytes)
            .unwrap();
        report_bytes.set_position(0);
        report_string.clear();
        report_bytes.read_to_string(&mut report_string).unwrap();
        eprintln!("{report_string}");
    }
}
