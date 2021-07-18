use crate::router::Route;
use zoon::*;
use std::fmt;
use evalexpr::eval;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, PartialEq)]
pub struct Expression {
    operand_a: f64,
    operator: String,
    operand_b: f64,
}

impl Expression {
    pub fn new(operand_a: impl Into<f64>, operator: impl Into<String>, operand_b: impl Into<f64>) -> Self {
        Self {
            operand_a: operand_a.into(),
            operator: operator.into(),
            operand_b: operand_b.into(),
        }
    }

    pub fn into_route(self) -> Route {
        let Self { operand_a, operator,  operand_b} = self;
        Route::Calc {
            operand_a,
            operator,
            operand_b,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.operand_a, self.operator, self.operand_b)
    }
}

// ------ ------
//    Statics
// ------ ------

#[static_ref]
pub fn expression() -> &'static Mutable<Option<Expression>> {
    Mutable::new(None)
}

// ------ ------
//    Signals
// ------ ------

fn result_signal() -> impl Signal<Item = Option<String>> {
    expression().signal_ref(|expr| {
        let expr = expr.as_ref()?;
        let result = eval(&expr.to_string()).ok()?;
        Some(result.to_string())
    })
}

// ------ ------
//   Commands
// ------ ------

pub fn set_expression(new_expression: Expression) {
    expression().set_neq(Some(new_expression));
}

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .item(result())
        .item(expressions())
}

fn result() -> impl Element {
    Row::new()
        .s(Spacing::new(15))
        .item(El::new().child("Result:"))
        .item_signal(result_signal())
}

fn expressions() -> impl Element {
    Row::new()
        .s(Spacing::new(35))
        .item(expression_link(Expression::new(3, '+', 7)))
        .item(expression_link(Expression::new(2, '^', 8)))
        .item(expression_link(Expression::new(10, '%', 3)))
}

fn expression_link(expression: Expression) -> impl Element {
    Link::new()
        .s(Font::new().color(NamedColor::Blue7))
        .label(expression.to_string())
        .to(expression.into_route())
}
