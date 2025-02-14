use crate::boon::parser::{
    lexer, parser, resolve_references, Input, ParseError, Parser, Span, Spanned, Token,
};
use crate::boon::platform::browser::{engine::Object, evaluator::evaluate};
use ariadne::{Config, Label, Report, ReportKind, Source};
use std::fmt;
use std::io::{Cursor, Read};
use std::sync::Arc;
use zoon::{eprintln, println, UnwrapThrowExt};

pub fn run(filename: &str, source_code: &str) -> Option<Arc<Object>> {
    println!("[Source Code ({filename})]");
    println!("{source_code}");

    let (tokens, errors) = lexer().parse(source_code).into_output_errors();
    if let Some(tokens) = tokens.as_ref() {
        println!("[Tokens]");
        println!("{tokens:?}");
    }
    if !errors.is_empty() {
        println!("[Lex Errors]");
    }
    report_errors(errors, filename, source_code);
    let Some(mut tokens) = tokens else {
        return None;
    };

    tokens.retain(|spanned_token| !matches!(spanned_token.node, Token::Comment(_)));

    let (ast, errors) = parser()
        .parse(
            tokens.map(Span::splat(source_code.len()), |Spanned { node, span }| {
                (node, span)
            }),
        )
        .into_output_errors();
    if let Some(ast) = ast.as_ref() {
        println!("[Abstract Syntax Tree]");
        println!("{ast:?}");
    }
    if !errors.is_empty() {
        println!("[Parse Errors]");
    }
    report_errors(errors, filename, source_code);
    let Some(ast) = ast else {
        return None;
    };

    let ast_with_reference_data = match resolve_references(ast) {
        Ok(ast_with_reference_data) => ast_with_reference_data,
        Err(errors) => {
            println!("[Reference Errors]");
            report_errors(errors, filename, source_code);
            return None;
        }
    };
    println!("[Abstract Syntax Tree with Reference Data]");
    println!("{ast_with_reference_data:#?}");

    let errors = match evaluate(ast_with_reference_data) {
        Ok(root_object) => return Some(root_object),
        Err(evaluation_error) => [evaluation_error],
    };
    println!("[Evaluation Errors]");
    report_errors(errors, filename, source_code);
    None
}

fn report_errors<'code, T: fmt::Display + 'code>(
    errors: impl IntoIterator<Item = ParseError<'code, T>>,
    filename: &str,
    source_code: &str,
) {
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
                    .with_message(error.reason().to_string()),
            )
            .finish()
            .write((filename, Source::from(source_code)), &mut report_bytes)
            .unwrap_throw();
        report_bytes.set_position(0);
        report_string.clear();
        report_bytes
            .read_to_string(&mut report_string)
            .unwrap_throw();
        eprintln!("{report_string}");
    }
}
