use std::fmt;
use chumsky::prelude::*;

pub use chumsky::Parser;

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
    SlashPath(Vec<&'code str>),
    PassedDotPath(Vec<&'code str>),
    DotPath(Vec<&'code str>),
    Passed,
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
            Self::Text(text) => write!(f, "{text}"),
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
            Self::SlashPath(parts) => write!(f, "{}", parts.join("/")),
            Self::PassedDotPath(parts) => write!(f, "PASSED.{}", parts.join(".")),
            Self::DotPath(parts) => write!(f, "{}", parts.join(".")),
            Self::Passed => write!(f, "PASSED"),
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

pub fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    // https://github.com/zesterer/chumsky/blob/main/tutorial.md
    let int = text::int(10)
        .map(|s: String| Expression::Literal(Literal::Number(s.parse().unwrap())))
        .padded();

    int
    // int.then_ignore(end())
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    List { items: Vec<Self> },
    Object { variables: Vec<Variable> },
    TaggedObject { tag: String, variables: Vec<Variable> },
    Map { entries: Vec<MapEntry> },
    Function { name: String, arguments: Vec<String>, body: Box<Self> },
    FunctionCall { name: String, arguments: Vec<Argument> },
    Alias { path: String },
    Link,
    LinkSetter { alias: String },
    Latest { inputs: Vec<Self> },
    Then { body: Box<Self> },
    When { arms: Vec<Arm> },
    While { arms: Vec<Arm> },
    Pipe { from: Box<Self>, to: Box<Self> },
    Skip,
    Block { variables: Vec<Variable>, output: Box<Self> },
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
pub enum Literal {
    Number(f64),
    Text(String),
    Tag(String),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct MapEntry {
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct Arm {
    pub pattern: Pattern,
    pub body: Expression,
}

#[derive(Debug)]
pub enum Pattern {
    Literal(Literal),
    List { items: Vec<Pattern> },
    Object { variables: Vec<PatternVariable> },
    TaggedObject { tag: String, variables: Vec<PatternVariable> },
    Map { entries: Vec<PatternMapEntry> },
    Alias { name: String },
    WildCard,
}

#[derive(Debug)]
pub struct PatternVariable {
    pub name: String,
    pub value: Option<Pattern>,
}

#[derive(Debug)]
pub struct PatternMapEntry {
    pub key: Pattern,
    pub value: Option<Pattern>,
}
