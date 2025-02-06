use std::fmt;
use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Token<'code> {
    Comment(&'code str),
    Number(f64),
    Pipe,
    Wildcard,
    Implies,
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
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenthesisOpen,
    ParenthesisClose,
    Colon,
    Comma,
    Dot,
    Newline,
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
            Self::Comment(comment) => write!(f, "{comment}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Pipe => write!(f, "|>"),
            Self::Wildcard => write!(f, "__"),
            Self::Implies => write!(f, "=>"),
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
            Self::BraceOpen => write!(f, "{{"),
            Self::BraceClose => write!(f, "}}"),
            Self::BracketOpen => write!(f, "["),
            Self::BracketClose => write!(f, "]"),
            Self::ParenthesisOpen => write!(f, "("),
            Self::ParenthesisClose => write!(f, ")"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Newline => write!(f, "\n"),
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

pub fn lexer<'code>() -> impl Parser<'code, &'code str, Vec<Token<'code>>, extra::Err<Rich<'code, char, SimpleSpan>>> {
    let comment = just("--")
        .ignore_then(
            any()
                .and_is(text::inline_whitespace().then(text::newline()).not())
                .repeated()
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
                .filter(char::is_ascii_lowercase)
                .or(any().filter(char::is_ascii_digit))
                .or(just('_'))
                .repeated()
        )
        .to_slice()
        .map(Token::SnakeCaseIdentifier);

    let pascal_case_identifier = any()
        .filter(char::is_ascii_uppercase)
        // @TODO replace with `.repeated().exactly(1)` once it works as expected?
        .then(any().filter(char::is_ascii_uppercase).not())
        .then(
            any()
                .filter(char::is_ascii_uppercase)
                .or(any().filter(char::is_ascii_lowercase))
                .or(any().filter(char::is_ascii_digit))
                .repeated()
        )
        .to_slice()
        .map(Token::PascalCaseIdentifier);

    let keyword = any()
        .filter(char::is_ascii_uppercase)
        .repeated()
        .at_least(2)
        .to_slice()
        .try_map(|keyword, span| {
            match keyword {
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
                _ => Err(Rich::custom(span, format!("Unknown keyword '{keyword}'")))
            }
        });

    let token = comment
        .or(number)
        .or(just("|>").to(Token::Pipe))
        .or(just("__").to(Token::Wildcard))
        .or(just("=>").to(Token::Implies))
        .or(just("=/=").to(Token::NotEqual))
        .or(just(">=").to(Token::GreaterOrEqual))
        .or(just('>').to(Token::Greater))
        .or(just("<=").to(Token::LessOrEqual))
        .or(just('<').to(Token::Less))
        .or(just('=').to(Token::Equal))
        .or(just('-').to(Token::Minus))
        .or(just('+').to(Token::Plus))
        .or(just('*').to(Token::Asterisk))
        .or(just('/').to(Token::Slash))
        .or(just('{').to(Token::BraceOpen))
        .or(just('}').to(Token::BraceClose))
        .or(just('[').to(Token::BracketOpen))
        .or(just(']').to(Token::BracketClose))
        .or(just('(').to(Token::ParenthesisOpen))
        .or(just(')').to(Token::ParenthesisClose))
        .or(just(':').to(Token::Colon))
        .or(just(',').to(Token::Comma))
        .or(just('.').to(Token::Dot))
        .or(text::newline().to(Token::Newline))
        .or(text)
        .or(snake_case_identifier)
        .or(pascal_case_identifier)
        .or(keyword);

    token
        .padded_by(text::inline_whitespace())
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
