use std::fmt;
use chumsky::prelude::*;

pub use chumsky::Parser;

enum Token<'code> {
    Comment(&'code str),
    Number(f64),
    Text(&'code str),
    SlashPath{ parts: Vec<&'code str> },
    DotPath { parts: Vec<&'code str>, passed: bool },
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
    Tag(&'code str),
    Identifier(&'code str),
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
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Comment(comment) => write!(f, "--{comment}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Text(text) => write!(f, "'{text}'"),
            Self::SlashPath { parts } => write!(f, "{}", parts.join("/")),
            Self::DotPath { parts, passed: _ } => write!(f, "{}", parts.join(".")),
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
            Self::Tag(tag) => write!(f, "{tag}"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
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
    let text = just('\'')
        .ignore_then(none_of('\'').repeated().to_slice())
        .then_ignore(just('\''))
        .map(Token::Text);


    // @TODO other parsers


    let token = number
        .or(text);

    token
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
