use chumsky::{input::ValueInput, prelude::*};

mod lexer;
use lexer::Token;

pub use chumsky::prelude::Parser;
pub use lexer::lexer;

pub type Span = SimpleSpan;
pub type ParseError<'code, T> = Rich<'code, T, Span>;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

pub fn make_input<'code>(
    end_of_input_span: Span,
    tokens: &'code [Spanned<Token<'code>>],
) -> impl ValueInput<'code, Token = Token<'code>, Span = Span> {
    tokens.map(end_of_input_span, |Spanned { node, span }| (node, span))
}

pub fn parser<'code, I, M>(
    make_input: M,
) -> impl Parser<'code, I, Spanned<Expression<'code>>, extra::Err<ParseError<'code, Token<'code>>>>
where
    I: ValueInput<'code, Token = Token<'code>, Span = Span>,
    M: Fn(Span, &'code [Spanned<Token<'code>>]) -> I + Clone + 'code,
{
    any().repeated().map_with(|token, extra| Spanned {
        node: Expression::Literal(Literal::Number(1.)),
        span: extra.span(),
    })
}

#[derive(Debug)]
pub enum Expression<'code> {
    Literal(Literal<'code>),
    List {
        items: Vec<Self>,
    },
    Object {
        variables: Vec<Variable<'code>>,
    },
    TaggedObject {
        tag: &'code str,
        variables: Vec<Variable<'code>>,
    },
    Map {
        entries: Vec<MapEntry<'code>>,
    },
    Function {
        name: &'code str,
        arguments: Vec<&'code str>,
        body: Box<Self>,
    },
    FunctionCall {
        path: Vec<&'code str>,
        arguments: Vec<Argument<'code>>,
    },
    Alias(Alias<'code>),
    Link,
    LinkSetter {
        alias: Alias<'code>,
    },
    Latest {
        inputs: Vec<Self>,
    },
    Then {
        body: Box<Self>,
    },
    When {
        arms: Vec<Arm<'code>>,
    },
    While {
        arms: Vec<Arm<'code>>,
    },
    Pipe {
        from: Box<Self>,
        to: Box<Self>,
    },
    Skip,
    Block {
        variables: Vec<Variable<'code>>,
        output: Box<Self>,
    },
    Comment,
    Equal {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    NotEqual {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Greater {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    GreaterOrEqual {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Less {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    LessOrEqual {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Negate {
        operand: Box<Self>,
    },
    Add {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Subtract {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Multiply {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
    Divide {
        operand_a: Box<Self>,
        operand_b: Box<Self>,
    },
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
    List {
        items: Vec<Pattern<'code>>,
    },
    Object {
        variables: Vec<PatternVariable<'code>>,
    },
    TaggedObject {
        tag: &'code str,
        variables: Vec<PatternVariable<'code>>,
    },
    Map {
        entries: Vec<PatternMapEntry<'code>>,
    },
    Alias {
        name: &'code str,
    },
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
