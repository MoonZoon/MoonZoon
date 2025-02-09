use chumsky::{input::ValueInput, prelude::*};

mod lexer;
pub use lexer::Token;

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
        let newlines = just(Token::Newline).repeated();
        let colon = just(Token::Colon);
        let slash = just(Token::Slash);
        let comma = just(Token::Comma);
        let bracket_round_open = just(Token::BracketRoundOpen);
        let bracket_round_close = just(Token::BracketRoundClose);
        let bracket_curly_open = just(Token::BracketCurlyOpen);
        let bracket_curly_close = just(Token::BracketCurlyClose);
        let bracket_square_open = just(Token::BracketSquareOpen);
        let bracket_square_close = just(Token::BracketSquareClose);

        let snake_case_identifier = select! { Token::SnakeCaseIdentifier(identifier) => identifier };
        let pascal_case_identifier = select! { Token::PascalCaseIdentifier(identifier) => identifier };

        let variable = group((snake_case_identifier, colon, expression.clone()))
            .map(|(name, _, value)| Variable { name, value });

        let expression_variable = variable
            .clone()
            .map(|variable| Expression::Variable(Box::new(variable)));

        let function_call = {
            let path = pascal_case_identifier
                .then_ignore(slash)
                .repeated()
                .collect::<Vec<_>>()
                .then(snake_case_identifier)
                .map(|(mut path, variable_name)| {
                    path.push(variable_name);
                    path
                });

            let argument = snake_case_identifier
                .then(group((colon, expression.clone())).or_not())
                .map_with(|(name, value), extra| {
                    let value = value.map(|(_, value)| value);
                    Spanned {
                        node: Argument { name, value },
                        span: extra.span(),
                    }
                });

            path
                .then(
                    argument
                        .separated_by(comma.ignored().or(newlines))
                        .collect()
                        .delimited_by(bracket_round_open.then(newlines), newlines.then(bracket_round_close))
                )
                .map(|(path, arguments)| {
                    Expression::FunctionCall { path, arguments }
                })
        };

        let number = select! { Token::Number(number) => Literal::Number(number) };
        let text = select! { Token::Text(text) => Literal::Text(text) };
        let tag = pascal_case_identifier.map(Literal::Tag);
        let literal = choice((number, text, tag)).map(Expression::Literal);

        let list = just(Token::List)
            .ignore_then(
                expression
                    .separated_by(comma.ignored().or(newlines))
                    .collect()
                    .delimited_by(bracket_curly_open.then(newlines), newlines.then(bracket_curly_close))
            )
            .map(|items| Expression::List { items });

        let object = variable
            .map_with(|variable, extra| { 
                Spanned {
                    node: variable,
                    span: extra.span()
                }
            })
            .separated_by(comma.ignored().or(newlines))
            .collect()
            .delimited_by(bracket_square_open.then(newlines), newlines.then(bracket_square_close))
            .map(|variables| Expression::Object { variables });

        let tagged_object = just(Token::BracketSquareOpen).ignore_then(todo());
        let map = just(Token::Map).ignore_then(todo());
        let function = just(Token::Function).ignore_then(todo());

        let expression = choice((
            expression_variable,
            function_call,
            list,
            object,
            tagged_object,
            map,
            literal,
            function,
        ));

        expression
            .map_with(|expression, extra| Spanned {
                node: expression,
                span: extra.span(),
            })
            .padded_by(newlines)
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
        items: Vec<Spanned<Self>>,
    },
    Object {
        variables: Vec<Spanned<Variable<'code>>>,
    },
    TaggedObject {
        tag: &'code str,
        variables: Vec<Spanned<Variable<'code>>>,
    },
    Map {
        entries: Vec<MapEntry<'code>>,
    },
    Function {
        name: &'code str,
        arguments: Vec<&'code str>,
        body: Box<Spanned<Self>>,
    },
    FunctionCall {
        path: Vec<&'code str>,
        arguments: Vec<Spanned<Argument<'code>>>,
    },
    Alias(Alias<'code>),
    Link,
    LinkSetter {
        alias: Alias<'code>,
    },
    Latest {
        inputs: Vec<Spanned<Self>>,
    },
    Then {
        body: Box<Spanned<Self>>,
    },
    When {
        arms: Vec<Arm<'code>>,
    },
    While {
        arms: Vec<Arm<'code>>,
    },
    Pipe {
        from: Box<Spanned<Self>>,
        to: Box<Spanned<Self>>,
    },
    Skip,
    Block {
        variables: Vec<Spanned<Variable<'code>>>,
        output: Box<Spanned<Self>>,
    },
    Comment,
    Equal {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    NotEqual {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Greater {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    GreaterOrEqual {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Less {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    LessOrEqual {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Negate {
        operand: Box<Spanned<Self>>,
    },
    Add {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Subtract {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Multiply {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
    },
    Divide {
        operand_a: Box<Spanned<Self>>,
        operand_b: Box<Spanned<Self>>,
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
    pub value: Option<Spanned<Expression<'code>>>,
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
