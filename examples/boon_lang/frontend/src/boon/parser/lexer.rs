use super::{ParseError, Spanned};
use chumsky::prelude::*;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'code> {
    BracketRoundOpen,
    BracketRoundClose,
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    Comment(&'code str),
    // @TODO decimal?
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

impl<'code> Token<'code> {
    pub fn into_cow_str(self) -> Cow<'code, str> {
        match self {
            Self::BracketRoundOpen => "(".into(),
            Self::BracketRoundClose => ")".into(),
            Self::BracketCurlyOpen => "{".into(),
            Self::BracketCurlyClose => "}".into(),
            Self::BracketSquareOpen => "[".into(),
            Self::BracketSquareClose => "]".into(),
            Self::Comment(comment) => comment.into(),
            Self::Number(number) => number.to_string().into(),
            Self::Pipe => "|>".into(),
            Self::Wildcard => "__".into(),
            Self::Implies => "=>".into(),
            Self::Colon => ":".into(),
            Self::Comma => ",".into(),
            Self::Dot => ".".into(),
            Self::Newline => "\n".into(),
            Self::NotEqual => "=/=".into(),
            Self::GreaterOrEqual => ">=".into(),
            Self::Greater => ">".into(),
            Self::LessOrEqual => "<=".into(),
            Self::Less => "<".into(),
            Self::Equal => "=".into(),
            Self::Minus => "-".into(),
            Self::Plus => "+".into(),
            Self::Asterisk => "*".into(),
            Self::Slash => "/".into(),
            Self::Text(text) => text.into(),
            Self::SnakeCaseIdentifier(identifier) => identifier.into(),
            Self::PascalCaseIdentifier(identifier) => identifier.into(),
            Self::List => "LIST".into(),
            Self::Map => "MAP".into(),
            Self::Function => "FUNCTION".into(),
            Self::Link => "LINK".into(),
            Self::Latest => "LATEST".into(),
            Self::Then => "THEN".into(),
            Self::When => "WHEN".into(),
            Self::While => "WHILE".into(),
            Self::Skip => "SKIP".into(),
            Self::Block => "BLOCK".into(),
            Self::Pass => "PASS".into(),
            Self::Passed => "PASSED".into(),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.into_cow_str())
    }
}

pub fn lexer<'code>()
-> impl Parser<'code, &'code str, Vec<Spanned<Token<'code>>>, extra::Err<ParseError<'code, char>>> {
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
    let number = just('-')
        .repeated()
        .at_most(1)
        .then(text::int(10).then(just('.').then(text::digits(10)).or_not()))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Number);

    // @TODO multiline indentation?
    // @TODO "raw" text or escape '? Idea: 'I am {name}' or #'I'm #{name}'#
    // - the same number of # at the beginning, at the end and before aliases
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
            _ => Err(ParseError::custom(
                span,
                format!("Unknown keyword '{keyword}'"),
            )),
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
            span: extra.span(),
            persistence: None,
        })
        .padded_by(text::inline_whitespace())
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
