use crate::*;

pub fn maybe_view(expression: impl Into<Option<Arc<Cow<'static, str>>>>) -> Option<impl Element> {
    if let Some(expression) = expression.into() {
        STORE.calc_page.expression.set(Some(expression));
    } else {
        if let Some(expression) = STORE.calc_page.expression.get_cloned() {
            ROUTER.silent_replace(Route::Calc { expression });
        }
    }
    Some(page_content())
}

fn page_content() -> impl Element {
    Column::new()
        .s(Gap::both(20))
        .item(result())
        .item(expressions())
}

fn result() -> impl Element {
    Row::new()
        .s(Gap::both(15))
        .item(El::new().child("Result:"))
        .item_signal(STORE.calc_page.expression.signal_ref(|expression| {
            let expression = expression.as_ref()?;
            let value = eval(expression).ok()?;
            Some(value.to_string())
        }))
}

fn expressions() -> impl Element {
    Row::new()
        .s(Gap::both(35))
        .item(expression_link("3 + 7"))
        .item(expression_link("2 ^ 8"))
        .item(expression_link("10 % 3"))
}

fn expression_link(expression: &'static str) -> impl Element {
    Link::new()
        .s(Font::new().color(color!("RoyalBlue")))
        .label(expression)
        .to(Route::Calc {
            expression: Arc::new(expression.into()),
        })
}
