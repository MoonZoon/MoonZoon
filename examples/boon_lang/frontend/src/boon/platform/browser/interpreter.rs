use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::io::{Cursor, Read};
use std::sync::Arc;

use ariadne::{Config, Label, Report, ReportKind, Source};
use chumsky::input::Stream;
use serde_json_any_key::MapIterToJson;
use zoon::{UnwrapThrowExt, WebStorage, eprintln, local_storage, println, serde_json};

use crate::boon::parser::{
    Expression, Input, ParseError, Parser, Span, Spanned, Token, lexer, parser,
    resolve_persistence, resolve_references,
};
use crate::boon::platform::browser::{
    engine::{ConstructContext, Object},
    evaluator::evaluate,
};

pub fn run(
    filename: &str,
    source_code: &str,
    states_local_storage_key: impl Into<Cow<'static, str>>,
    old_code_local_storage_key: impl Into<Cow<'static, str>>,
    old_span_id_pairs_local_storage_key: impl Into<Cow<'static, str>>,
) -> Option<(Arc<Object>, ConstructContext)> {
    let states_local_storage_key = states_local_storage_key.into();
    let old_code_local_storage_key = old_code_local_storage_key.into();
    let old_span_id_pairs_local_storage_key = old_span_id_pairs_local_storage_key.into();

    let old_source_code = local_storage().get::<String>(&old_code_local_storage_key);
    let old_ast = if let Some(Ok(old_source_code)) = &old_source_code {
        parse_old(filename, old_source_code)
    } else {
        None
    };

    println!("[Source Code ({filename})]");
    println!("{source_code}");

    let (tokens, errors) = lexer().parse(source_code).into_output_errors();
    if let Some(tokens) = tokens.as_ref() {
        // println!("[Tokens]");
        // println!("{tokens:?}");
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
        .parse(tokens.map(
            Span::splat(source_code.len()),
            |Spanned {
                 node,
                 span,
                 persistence: _,
             }| { (node, span) },
        ))
        .into_output_errors();
    if let Some(ast) = ast.as_ref() {
        // println!("[Abstract Syntax Tree]");
        // println!("{ast:?}");
    }
    if !errors.is_empty() {
        println!("[Parse Errors]");
    }
    report_errors(errors, filename, source_code);
    let Some(ast) = ast else {
        return None;
    };

    let ast = match resolve_references(ast) {
        Ok(ast) => ast,
        Err(errors) => {
            println!("[Reference Errors]");
            report_errors(errors, filename, source_code);
            return None;
        }
    };
    // println!("[Abstract Syntax Tree with Reference Data]");
    // println!("{ast:?}");

    let (ast, new_span_id_pairs) =
        match resolve_persistence(ast, old_ast, &old_span_id_pairs_local_storage_key) {
            Ok(ast) => ast,
            Err(errors) => {
                println!("[Persistence Errors]");
                report_errors(errors, filename, source_code);
                return None;
            }
        };
    println!("[Abstract Syntax Tree with Reference Data and Persistence]");
    println!("{ast:#?}");

    let (evaluation_result, errors) = match evaluate(ast, states_local_storage_key.clone()) {
        Ok(evaluation_result) => (Some(evaluation_result), vec![]),
        Err(evaluation_error) => (None, vec![evaluation_error]),
    };
    if !errors.is_empty() {
        println!("[Evaluation Errors]");
    }
    report_errors(errors, filename, source_code);

    if evaluation_result.is_some() {
        if let Err(error) = local_storage().insert(&old_code_local_storage_key, &source_code) {
            eprintln!("Failed to store source code as old source code: {error:#?}");
        }

        if let Err(error) = local_storage().insert(
            &old_span_id_pairs_local_storage_key,
            &new_span_id_pairs.to_json_map().unwrap(),
        ) {
            eprintln!("Failed to store Span-PersistenceId pairs: {error:#}");
        }

        if let Some(states) =
            local_storage().get::<BTreeMap<String, serde_json::Value>>(&states_local_storage_key)
        {
            let mut states = states.expect("Failed to deseralize states");
            let persistent_ids = new_span_id_pairs
                .values()
                .map(|id| id.to_string())
                .collect::<HashSet<_>>();
            states.retain(|id, _| persistent_ids.contains(id));
            if let Err(error) = local_storage().insert(&states_local_storage_key, &states) {
                eprintln!("Failed to store states after removing old ones: {error:#?}");
            }
        }
    }

    evaluation_result
}

fn parse_old<'filename, 'old_code>(
    filename: &'filename str,
    source_code: &'old_code str,
) -> Option<Vec<Spanned<Expression<'old_code>>>> {
    let (tokens, errors) = lexer().parse(source_code).into_output_errors();
    if !errors.is_empty() {
        println!("[OLD Lex Errors]");
    }
    report_errors(errors, filename, source_code);
    let Some(mut tokens) = tokens else {
        return None;
    };

    tokens.retain(|spanned_token| !matches!(spanned_token.node, Token::Comment(_)));

    let (ast, errors) = parser()
        .parse(Stream::from_iter(tokens).map(
            Span::splat(source_code.len()),
            |Spanned {
                 node,
                 span,
                 persistence: _,
             }| { (node, span) },
        ))
        .into_output_errors();
    if !errors.is_empty() {
        println!("[OLD Parse Errors]");
    }
    report_errors(errors, filename, source_code);
    let Some(ast) = ast else {
        return None;
    };

    let ast_with_reference_data = match resolve_references(ast) {
        Ok(ast_with_reference_data) => ast_with_reference_data,
        Err(errors) => {
            println!("[OLD Reference Errors]");
            report_errors(errors, filename, source_code);
            return None;
        }
    };
    Some(ast_with_reference_data)
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
