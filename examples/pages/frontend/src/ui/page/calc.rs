use crate::*;

#[derive(Clone)]
pub struct CalcPage {
    expression: Option<Expression>,
}

impl CalcPage {
    pub fn new(expression: Option<Expression>) -> impl Element {
        Self { expression }.root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Gap::both(20))
            .item(self.result())
            .item(self.expressions())
    }

    fn result(&self) -> impl Element {
        Row::new()
            .s(Gap::both(15))
            .item(El::new().child("Result:"))
            .item(self.expression.as_ref().and_then(|Expression(expression)| {
                let value = eval(expression).ok()?;
                Some(value.to_string())
            }))
    }

    fn expressions(&self) -> impl Element {
        Row::new()
            .s(Gap::both(35))
            .item(self.expression_link("3 + 7"))
            .item(self.expression_link("2 ^ 8"))
            .item(self.expression_link("10 % 3"))
    }

    fn expression_link(&self, expression: &'static str) -> impl Element {
        Link::new()
            .s(Font::new().color(color!("RoyalBlue")))
            .label(expression)
            .to(Route::Calc {
                expression: Arc::new(expression.into()),
            })
    }
}
