// @TODO remove
#![allow(dead_code)]

use chumsky::{input::ValueInput, pratt::*, prelude::*};
use std::fmt;

mod lexer;
pub use lexer::{Token, lexer};

mod scope_resolver;
pub use scope_resolver::{Referenceables, resolve_references};

mod persistence_resolver;
pub use persistence_resolver::{Persistence, PersistenceId, resolve_persistence};

pub use chumsky::prelude::{Input, Parser};

pub type Span = SimpleSpan;
pub type ParseError<'code, T> = Rich<'code, T, Span>;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub persistence: Option<Persistence>,
    pub node: T,
}

pub fn parser<'code, I>()
-> impl Parser<'code, I, Vec<Spanned<Expression<'code>>>, extra::Err<ParseError<'code, Token<'code>>>>
where
    I: ValueInput<'code, Token = Token<'code>, Span = Span>,
{
    let newlines = just(Token::Newline).repeated();

    recursive(|expression| {
        let colon = just(Token::Colon);
        let slash = just(Token::Slash);
        let comma = just(Token::Comma);
        let dot = just(Token::Dot);
        let bracket_round_open = just(Token::BracketRoundOpen);
        let bracket_round_close = just(Token::BracketRoundClose);
        let bracket_curly_open = just(Token::BracketCurlyOpen);
        let bracket_curly_close = just(Token::BracketCurlyClose);
        let bracket_square_open = just(Token::BracketSquareOpen);
        let bracket_square_close = just(Token::BracketSquareClose);

        let snake_case_identifier =
            select! { Token::SnakeCaseIdentifier(identifier) => identifier };
        let pascal_case_identifier =
            select! { Token::PascalCaseIdentifier(identifier) => identifier };

        let variable =
            group((snake_case_identifier, colon, expression.clone())).map(|(name, _, value)| {
                Variable {
                    name,
                    is_referenced: false,
                    value,
                }
            });

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
                        node: Argument {
                            name,
                            is_referenced: false,
                            value,
                        },
                        span: extra.span(),
                        persistence: None,
                    }
                });

            path.then(
                argument
                    .separated_by(comma.ignored().or(newlines))
                    .collect()
                    .delimited_by(
                        bracket_round_open.then(newlines),
                        newlines.then(bracket_round_close),
                    ),
            )
            .map(|(path, arguments)| Expression::FunctionCall { path, arguments })
        };

        let number = select! { Token::Number(number) => Literal::Number(number) };
        let text = select! { Token::Text(text) => Literal::Text(text) };
        let tag = pascal_case_identifier.map(Literal::Tag);

        let literal = choice((number, text, tag));
        let expression_literal = literal.map(Expression::Literal);

        let list = just(Token::List)
            .ignore_then(
                expression
                    .clone()
                    .separated_by(comma.ignored().or(newlines))
                    .collect()
                    .delimited_by(
                        bracket_curly_open.then(newlines),
                        newlines.then(bracket_curly_close),
                    ),
            )
            .map(|items| Expression::List { items });

        let object = variable
            .map_with(|variable, extra| Spanned {
                node: variable,
                span: extra.span(),
                persistence: None,
            })
            .separated_by(comma.ignored().or(newlines))
            .collect()
            .delimited_by(
                bracket_square_open.then(newlines),
                newlines.then(bracket_square_close),
            )
            .map(|variables| Object { variables });

        let expression_object = object.clone().map(Expression::Object);

        let tagged_object = pascal_case_identifier
            .then(object)
            .map(|(tag, object)| Expression::TaggedObject { tag, object });

        let alias = {
            let alias_with_passed = just(Token::Passed)
                .ignore_then(
                    snake_case_identifier
                        .separated_by(dot)
                        .allow_leading()
                        .collect::<Vec<_>>(),
                )
                .map(|extra_parts| Alias::WithPassed { extra_parts });

            let alias_without_passed = snake_case_identifier
                .separated_by(dot)
                .at_least(1)
                .collect::<Vec<_>>()
                .map(|parts| Alias::WithoutPassed {
                    parts,
                    referenceables: None,
                });

            alias_with_passed.or(alias_without_passed)
        };

        let expression_alias = alias.map(Expression::Alias);

        let map = {
            let key = literal
                .map(MapEntryKey::Literal)
                .or(alias.map(MapEntryKey::Alias))
                .map_with(|key, extra| Spanned {
                    span: extra.span(),
                    node: key,
                    persistence: None,
                });

            let key_value_pair = group((key, colon, expression.clone()))
                .map(|(key, _, value)| MapEntry { key, value });

            just(Token::Map)
                .ignore_then(
                    key_value_pair
                        .separated_by(comma.ignored().or(newlines))
                        .collect()
                        .delimited_by(
                            bracket_curly_open.then(newlines),
                            newlines.then(bracket_curly_close),
                        ),
                )
                .map(|entries| Expression::Map { entries })
        };

        let function = {
            let parameters = snake_case_identifier
                .map_with(|parameter_name, extra| Spanned {
                    node: parameter_name,
                    span: extra.span(),
                    persistence: None,
                })
                .separated_by(comma.ignored().or(newlines))
                .collect()
                .delimited_by(
                    bracket_round_open.then(newlines),
                    newlines.then(bracket_round_close),
                );

            just(Token::Function)
                .ignore_then(snake_case_identifier)
                .then(parameters)
                .then(expression.clone().delimited_by(
                    bracket_curly_open.then(newlines),
                    newlines.then(bracket_curly_close),
                ))
                .map(|((name, parameters), body)| Expression::Function {
                    name,
                    parameters,
                    body: Box::new(body),
                })
        };

        let link = just(Token::Link);
        let link_expression = link.map(|_| Expression::Link);

        let link_setter = link.ignore_then(
            alias
                .delimited_by(
                    bracket_curly_open.then(newlines),
                    newlines.then(bracket_curly_close),
                )
                .map_with(|alias, extra| Expression::LinkSetter {
                    alias: Spanned {
                        span: extra.span(),
                        node: alias,
                        persistence: None,
                    },
                }),
        );

        let latest = just(Token::Latest)
            .ignore_then(
                expression
                    .clone()
                    .separated_by(comma.ignored().or(newlines))
                    .collect()
                    .delimited_by(
                        bracket_curly_open.then(newlines),
                        newlines.then(bracket_curly_close),
                    ),
            )
            .map(|inputs| Expression::Latest { inputs });

        let then = just(Token::Then).ignore_then(
            expression
                .clone()
                .delimited_by(
                    bracket_curly_open.then(newlines),
                    newlines.then(bracket_curly_close),
                )
                .map(|body| Expression::Then {
                    body: Box::new(body),
                }),
        );

        let when = just(Token::When).ignore_then(todo());
        let while_ = just(Token::While).ignore_then(todo());

        let skip = select! { Token::Skip => Expression::Skip };

        let block = just(Token::Block).ignore_then(todo());

        // @TODO PASS, a part of function calls?
        // @TODO when, while
        // @TODO comparator + arithmetic operator (in pratt, update pipe binding power accordingly)
        // @TODO text interpolation with {}, what about escaping {} and ''?
        // @TODO parse todo_mvc.bn

        let nested = bracket_round_open
            .ignore_then(expression)
            .then_ignore(bracket_round_close);

        let expression = choice((
            expression_variable,
            function_call,
            list,
            expression_object,
            tagged_object,
            map,
            expression_literal,
            function,
            expression_alias,
            link_setter,
            link_expression,
            latest,
            then,
            when,
            while_,
            skip,
            block,
        ));

        expression
            .map_with(|expression, extra| Spanned {
                node: expression,
                span: extra.span(),
                persistence: None,
            })
            .or(nested)
            .padded_by(newlines)
            .pratt((infix(left(1), just(Token::Pipe), |l, _, r, extra| {
                let expression = Expression::Pipe {
                    from: Box::new(l),
                    to: Box::new(r),
                };
                Spanned {
                    span: extra.span(),
                    node: expression,
                    persistence: None,
                }
            }),))
    })
    .repeated()
    .collect()
    .padded_by(newlines)
}

// @TODO not everything is expression, FUNCTIONs can be defined only in the root, etc.
#[derive(Debug)]
pub enum Expression<'code> {
    Variable(Box<Variable<'code>>),
    Literal(Literal<'code>),
    List {
        items: Vec<Spanned<Self>>,
    },
    Object(Object<'code>),
    TaggedObject {
        tag: &'code str,
        object: Object<'code>,
    },
    Map {
        entries: Vec<MapEntry<'code>>,
    },
    Function {
        name: &'code str,
        parameters: Vec<Spanned<&'code str>>,
        body: Box<Spanned<Self>>,
    },
    FunctionCall {
        path: Vec<&'code str>,
        arguments: Vec<Spanned<Argument<'code>>>,
    },
    Alias(Alias<'code>),
    LinkSetter {
        alias: Spanned<Alias<'code>>,
    },
    Link,
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
    Comparator(Comparator<'code>),
    ArithmeticOperator(ArithmeticOperator<'code>),
}

#[derive(Debug)]
pub enum Comparator<'code> {
    Equal {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    NotEqual {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    Greater {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    GreaterOrEqual {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    Less {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    LessOrEqual {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
}

#[derive(Debug)]
pub enum ArithmeticOperator<'code> {
    Negate {
        operand: Box<Spanned<Expression<'code>>>,
    },
    Add {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    Subtract {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    Multiply {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
    Divide {
        operand_a: Box<Spanned<Expression<'code>>>,
        operand_b: Box<Spanned<Expression<'code>>>,
    },
}

#[derive(Debug)]
pub struct Object<'code> {
    pub variables: Vec<Spanned<Variable<'code>>>,
}

#[derive(Debug)]
pub struct Variable<'code> {
    pub name: &'code str,
    pub is_referenced: bool,
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
    pub key: Spanned<MapEntryKey<'code>>,
    pub value: Spanned<Expression<'code>>,
}

#[derive(Debug)]
pub enum MapEntryKey<'code> {
    Literal(Literal<'code>),
    Alias(Alias<'code>),
}

#[derive(Debug)]
pub struct Argument<'code> {
    pub name: &'code str,
    pub is_referenced: bool,
    pub value: Option<Spanned<Expression<'code>>>,
}

#[derive(Debug)]
pub enum Alias<'code> {
    WithoutPassed {
        parts: Vec<&'code str>,
        referenceables: Option<Referenceables<'code>>,
    },
    WithPassed {
        extra_parts: Vec<&'code str>,
    },
}

impl<'code> fmt::Display for Alias<'code> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WithPassed { extra_parts } => {
                let passed = Token::Passed;
                if extra_parts.is_empty() {
                    write!(f, "{passed}")
                } else {
                    write!(f, "{passed}.{}", extra_parts.join("."))
                }
            }
            Self::WithoutPassed {
                parts,
                referenceables: _,
            } => {
                write!(f, "{}", parts.join("."))
            }
        }
    }
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
