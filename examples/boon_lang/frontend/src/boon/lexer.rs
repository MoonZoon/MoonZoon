use std::fmt;
use chumsky::prelude::*;

pub use chumsky::Parser;

#[derive(Debug, Clone, Copy)]
pub enum Token<'code> {
    Comment(&'code str),
    Number(f64),
    Text(&'code str),
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
    PascalCaseIdentifier(&'code str),
    SnakeCaseIdentifier(&'code str),
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
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Comment(comment) => write!(f, "--{comment}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Text(text) => write!(f, "'{text}'"),
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
            Self::PascalCaseIdentifier(identifier) => write!(f, "{identifier}"),
            Self::SnakeCaseIdentifier(identifier) => write!(f, "{identifier}"),
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
        }
   }
}

pub fn lexer<'code>() -> impl Parser<'code, &'code str, Vec<Token<'code>>, extra::Err<Rich<'code, char, SimpleSpan>>> {
    let comment = just("--")
        .then(any().and_is(text::newline().not()).repeated())
        .padded();

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

    let pascal_case_identifier = any()
        .filter(char::is_ascii_uppercase)
        .then(
            any()
                .filter(char::is_ascii_uppercase)
                .or(any().filter(char::is_ascii_lowercase))
                .or(any().filter(char::is_ascii_digit))
                .repeated()
        )
        .to_slice()
        .map(Token::PascalCaseIdentifier);

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

    let token = number
        .or(text)
        .or(just("LIST").to(Token::List))
        .or(just("MAP").to(Token::Map))
        .or(just("FUNCTION").to(Token::Function))
        .or(just("LINK").to(Token::Link))
        .or(just("LATEST").to(Token::Latest))
        .or(just("THEN").to(Token::Then))
        .or(just("WHEN").to(Token::When))
        .or(just("WHILE").to(Token::While))
        .or(just("SKIP").to(Token::Skip))
        .or(just("BLOCK").to(Token::Block))
        .or(just("PASS").to(Token::Pass))
        .or(just("PASSED").to(Token::Passed))
        .or(pascal_case_identifier)
        .or(snake_case_identifier)
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
        .or(just('.').to(Token::Dot));

    token
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
