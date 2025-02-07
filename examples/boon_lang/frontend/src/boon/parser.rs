use chumsky::{input::ValueInput, prelude::*};

mod lexer;
use lexer::Token;

pub use chumsky::prelude::{Parser, Input};
pub use lexer::lexer;

pub type Span = SimpleSpan;
pub type ParseError<'code, T> = Rich<'code, T, Span>;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

pub fn parser<'code, I>() -> impl Parser<'code, I, Vec<Spanned<Expression<'code>>>, extra::Err<ParseError<'code, Token<'code>>>>
where
    I: ValueInput<'code, Token = Token<'code>, Span = Span>,
{
    recursive(|expression| {
        let function = just(Token::Function).ignore_then(todo());

        let comment = select!(Token::Comment(_)).ignored();
        let newline = just(Token::Newline).ignored();

        let colon = just(Token::Colon);
        let slash = just(Token::Slash);

        let snake_case_identifier = select! { Token::SnakeCaseIdentifier(identifier) => identifier };
        let pascal_case_identifier = select! { Token::PascalCaseIdentifier(identifier) => identifier };

        let number = select! { Token::Number(number) => Literal::Number(number) };
        let text = select! { Token::Text(text) => Literal::Text(text) };
        let tag = pascal_case_identifier.then_ignore(slash.not()).map(|identifier| {
            Literal::Tag(identifier)
        });
        let literal = choice((number, text, tag)).map(Expression::Literal);

        let variable = group((snake_case_identifier, colon, expression))
            .map(|(name, _, value)| Expression::Variable(Box::new(Variable { name, value })));

        let expression = choice((
            variable,
            function,
            literal,
        ));

        expression
            .map_with(|expression, extra| Spanned {
                node: expression,
                span: extra.span(),
            })
            .padded_by(newline.or(comment).repeated())
    })
    .repeated()
    .collect()
}

// @TODO not everything is expression, FUNCTIONs can be defined only in the root, etc. 
#[derive(Debug)]
pub enum Expression<'code> {
    Variable(Box<Variable<'code>>),
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
pub struct Variable<'code> {
    pub name: &'code str,
    pub value: Spanned<Expression<'code>>,
}

#[derive(Debug)]
pub enum Literal<'code> {
    Number(f64),
    Text(&'code str),
    Tag(&'code str),
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
