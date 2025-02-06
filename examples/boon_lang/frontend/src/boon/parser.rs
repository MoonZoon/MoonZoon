use std::fmt;
use chumsky::prelude::*;

enum Token<'code> {
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
    Pipe,
    Skip,
    Block,
    Pass,
    SlashPath{ parts: Vec<&'code str> },
    DotPath { parts: Vec<&'code str>, passed: bool },
    Passed,
    Tag(&'code str),
    Identifier(&'code str),
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
            Self::List => write!(f, "LIST"),
            Self::Map => write!(f, "MAP"),
            Self::Function => write!(f, "FUNCTION"),
            Self::Link => write!(f, "LINK"),
            Self::Latest => write!(f, "LATEST"),
            Self::Then => write!(f, "THEN"),
            Self::When => write!(f, "WHEN"),
            Self::While => write!(f, "WHILE"),
            Self::Pipe => write!(f, "|>"),
            Self::Skip => write!(f, "SKIP"),
            Self::Block => write!(f, "BLOCK"),
            Self::Pass => write!(f, "PASS"),
            Self::SlashPath { parts } => write!(f, "{}", parts.join("/")),
            Self::DotPath { parts, passed: _ } => write!(f, "{}", parts.join(".")),
            Self::Passed => write!(f, "PASSED"),
            Self::Tag(tag) => write!(f, "{tag}"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
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

pub fn parser<'code>() -> impl Parser<'code, &'code str, Expression<'code>, extra::Err<Rich<'code, char, SimpleSpan>>> {
    // https://github.com/zesterer/chumsky/blob/main/tutorial.md
    let int = text::int(10)
        .map(|s: &str| Expression::Literal(Literal::Number(s.parse().unwrap())))
        .padded();

    int.then_ignore(any().repeated())
}

#[derive(Debug)]
pub enum Expression<'code> {
    Literal(Literal<'code>),
    List { items: Vec<Self> },
    Object { variables: Vec<Variable<'code>> },
    TaggedObject { tag: &'code str, variables: Vec<Variable<'code>> },
    Map { entries: Vec<MapEntry<'code>> },
    Function { name: &'code str, arguments: Vec<&'code str>, body: Box<Self> },
    FunctionCall { path: Vec<&'code str>, arguments: Vec<Argument<'code>> },
    Alias(Alias<'code>),
    Link,
    LinkSetter { alias: Alias<'code> },
    Latest { inputs: Vec<Self> },
    Then { body: Box<Self> },
    When { arms: Vec<Arm<'code>> },
    While { arms: Vec<Arm<'code>> },
    Pipe { from: Box<Self>, to: Box<Self> },
    Skip,
    Block { variables: Vec<Variable<'code>>, output: Box<Self> },
    Comment,
    Equal { operand_a: Box<Self>, operand_b: Box<Self> },
    NotEqual { operand_a: Box<Self>, operand_b: Box<Self> },
    Greater { operand_a: Box<Self>, operand_b: Box<Self> },
    GreaterOrEqual { operand_a: Box<Self>, operand_b: Box<Self> },
    Less { operand_a: Box<Self>, operand_b: Box<Self> },
    LessOrEqual { operand_a: Box<Self>, operand_b: Box<Self> },
    Negate { operand: Box<Self> },
    Add { operand_a: Box<Self>, operand_b: Box<Self> },
    Subtract { operand_a: Box<Self>, operand_b: Box<Self> },
    Multiply { operand_a: Box<Self>, operand_b: Box<Self> },
    Divide { operand_a: Box<Self>, operand_b: Box<Self> }
}

#[derive(Debug)]
pub enum Literal<'code> {
    Number(f64),
    Text(&'code str),
    Tag(&'code str),
}

#[derive(Debug)]
pub struct Variable<'code> {
    pub name: &'code str,
    pub value: Expression<'code>,
}

#[derive(Debug)]
pub struct MapEntry<'code> {
    pub key: Expression<'code>,
    pub value: Expression<'code>,
}

#[derive(Debug)]
pub struct Argument<'code> {
    pub name: &'code str,
    pub value: Option<Expression<'code>>,
}

#[derive(Debug)]
pub struct Alias<'code> {
    pub parts: Vec<&'code str>,
    pub passed: bool,
}

#[derive(Debug)]
pub struct Arm<'code> {
    pub pattern: Pattern<'code>,
    pub body: Expression<'code>,
}

#[derive(Debug)]
pub enum Pattern<'code> {
    Literal(Literal<'code>),
    List { items: Vec<Pattern<'code>> },
    Object { variables: Vec<PatternVariable<'code>> },
    TaggedObject { tag: &'code str, variables: Vec<PatternVariable<'code>> },
    Map { entries: Vec<PatternMapEntry<'code>> },
    Alias { name: &'code str },
    WildCard,
}

#[derive(Debug)]
pub struct PatternVariable<'code> {
    pub name: &'code str,
    pub value: Option<Pattern<'code>>,
}

#[derive(Debug)]
pub struct PatternMapEntry<'code> {
    pub key: Pattern<'code>,
    pub value: Option<Pattern<'code>>,
}
