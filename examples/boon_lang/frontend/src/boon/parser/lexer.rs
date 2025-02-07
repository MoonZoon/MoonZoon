use chumsky::prelude::*;
use std::fmt;
use super::{Spanned, ParseError};

#[derive(Debug, Clone, Copy)]
pub enum Token<'code> {
    BracketRoundOpen,
    BracketRoundClose,
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    Comment(&'code str),
    Number(f64),
    Pipe,
    Wildcard,
    Implies,
    Colon,
    Comma,
    Dot,
    Newline,
    NotEqual,
    GreaterOrEqual,
    Greater,
    LessOrEqual,
    Less,
    Equal,
    Minus,
    Plus,
    Asterisk,
    Slash,
    Text(&'code str),
    SnakeCaseIdentifier(&'code str),
    PascalCaseIdentifier(&'code str),
    List,
    Map,
    Function,
    Link,
    Latest,
    Then,
    When,
    While,
    Skip,
    Block,
    Pass,
    Passed,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BracketRoundOpen => write!(f, "("),
            Self::BracketRoundClose => write!(f, ")"),
            Self::BracketCurlyOpen => write!(f, "{{"),
            Self::BracketCurlyClose => write!(f, "}}"),
            Self::BracketSquareOpen => write!(f, "["),
            Self::BracketSquareClose => write!(f, "]"),
            Self::Comment(comment) => write!(f, "{comment}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Pipe => write!(f, "|>"),
            Self::Wildcard => write!(f, "__"),
            Self::Implies => write!(f, "=>"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Newline => write!(f, "\n"),
            Self::NotEqual => write!(f, "=/="),
            Self::GreaterOrEqual => write!(f, ">="),
            Self::Greater => write!(f, ">"),
            Self::LessOrEqual => write!(f, "<="),
            Self::Less => write!(f, "<"),
            Self::Equal => write!(f, "="),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Text(text) => write!(f, "'{text}'"),
            Self::SnakeCaseIdentifier(identifier) => write!(f, "{identifier}"),
            Self::PascalCaseIdentifier(identifier) => write!(f, "{identifier}"),
            Self::List => write!(f, "LIST"),
            Self::Map => write!(f, "MAP"),
            Self::Function => write!(f, "FUNCTION"),
            Self::Link => write!(f, "LINK"),
            Self::Latest => write!(f, "LATEST"),
            Self::Then => write!(f, "THEN"),
            Self::When => write!(f, "WHEN"),
            Self::While => write!(f, "WHILE"),
            Self::Skip => write!(f, "SKIP"),
            Self::Block => write!(f, "BLOCK"),
            Self::Pass => write!(f, "PASS"),
            Self::Passed => write!(f, "PASSED"),
        }
    }
}

pub fn lexer<'code>(
) -> impl Parser<'code, &'code str, Vec<Spanned<Token<'code>>>, extra::Err<ParseError<'code>>> {
    let bracket = choice((
        just('(').to(Token::BracketRoundOpen),
        just(')').to(Token::BracketRoundClose),
        just('{').to(Token::BracketCurlyOpen),
        just('}').to(Token::BracketCurlyClose),
        just('[').to(Token::BracketSquareOpen),
        just(']').to(Token::BracketSquareClose),
    ));

    let comparator = choice((
        just("=/=").to(Token::NotEqual),
        just(">=").to(Token::GreaterOrEqual),
        just('>').to(Token::Greater),
        just("<=").to(Token::LessOrEqual),
        just('<').to(Token::Less),
        just('=').to(Token::Equal),
    ));

    let arithmetic_operator_or_path_separator = choice((
        just('-').to(Token::Minus),
        just('+').to(Token::Plus),
        just('*').to(Token::Asterisk),
        just('/').to(Token::Slash),
    ));

    let comment = just("--")
        .ignore_then(
            any()
                .and_is(text::inline_whitespace().then(text::newline()).not())
                .repeated(),
        )
        .to_slice()
        .map(Token::Comment);

    // @TODO support number format like 1_000?
    let number = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Number);

    // @TODO multiline indentation?
    // @TODO "raw" text or escape '?
    let text = just('\'')
        .ignore_then(none_of('\'').repeated().to_slice())
        .then_ignore(just('\''))
        .map(Token::Text);

    let snake_case_identifier = any()
        .filter(char::is_ascii_lowercase)
        .then(
            any()
                .filter(|character: &char| {
                    *character == '_'
                        || character.is_ascii_lowercase()
                        || character.is_ascii_digit()
                })
                .repeated(),
        )
        .to_slice()
        .map(Token::SnakeCaseIdentifier);

    let pascal_case_identifier = any()
        .filter(char::is_ascii_uppercase)
        .then(any().filter(|character: &char| character.is_ascii_lowercase() || character.is_ascii_uppercase() || character.is_ascii_digit()).repeated())
        .to_slice()
        .try_map(|identifier: &str, span| {
            if identifier.len() == 1 || identifier.chars().rev().any(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit()
            }) {
                Ok(Token::PascalCaseIdentifier(identifier))
            } else {
                Err(ParseError::custom(span, format!("PascalCase identifier has to contain at least one digit or lowercase character. Identifier: '{identifier}'")))
            }
        });

    let keyword = any()
        .filter(char::is_ascii_uppercase)
        .repeated()
        .at_least(2)
        .to_slice()
        .try_map(|keyword, span| match keyword {
            "LIST" => Ok(Token::List),
            "MAP" => Ok(Token::Map),
            "FUNCTION" => Ok(Token::Function),
            "LINK" => Ok(Token::Link),
            "LATEST" => Ok(Token::Latest),
            "THEN" => Ok(Token::Then),
            "WHEN" => Ok(Token::When),
            "WHILE" => Ok(Token::While),
            "SKIP" => Ok(Token::Skip),
            "BLOCK" => Ok(Token::Block),
            "PASS" => Ok(Token::Pass),
            "PASSED" => Ok(Token::Passed),
            _ => Err(ParseError::custom(span, format!("Unknown keyword '{keyword}'"))),
        });

    let token = choice((
        bracket,
        comment,
        number,
        just("|>").to(Token::Pipe),
        just("__").to(Token::Wildcard),
        just("=>").to(Token::Implies),
        just(':').to(Token::Colon),
        just(',').to(Token::Comma),
        just('.').to(Token::Dot),
        text::newline().to(Token::Newline),
        comparator,
        arithmetic_operator_or_path_separator,
        text,
        snake_case_identifier,
        pascal_case_identifier,
        keyword,
    ));

    token
        .map_with(|token, extra| Spanned {
            node: token,
            span: extra.span()
        })
        .padded_by(text::inline_whitespace())
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
