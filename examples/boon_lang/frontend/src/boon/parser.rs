use chumsky::prelude::*;

pub use chumsky::Parser;

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
    List { items: Vec<Expression> },
    Object { variables: Vec<Variable> },
    TaggedObject { tag: String, variables: Vec<Variable> },
    Map { entries: Vec<MapEntry> },
    Function { name: String, arguments: Vec<String>, body: Box<Expression> },
    FunctionCall { name: String, arguments: Vec<Argument> },
    Alias { path: String },
    Link,
    LinkSetter { alias: String },
    Latest { inputs: Vec<Expression> },
    Then { body: Box<Expression> },
    When { arms: Vec<Arm> },
    While { arms: Vec<Arm> },
    Pipe { from: Box<Expression>, to: Box<Expression> },
    Skip,
    Block { variables: Vec<Variable>, output: Box<Expression> },
    Comment,
    Negation { operand: Box<Expression> },
    Addition { operand_a: Box<Expression>, operand_b: Box<Expression> },
    Subtraction { operand_a: Box<Expression>, operand_b: Box<Expression> },
    Multiplication { operand_a: Box<Expression>, operand_b: Box<Expression> },
    Division { operand_a: Box<Expression>, operand_b: Box<Expression> }
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
