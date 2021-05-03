use zoon::*;
use rand::prelude::*;
use std::{sync::Arc, iter::repeat_with, array};

// ------ ------
//    Statics 
// ------ ------

static ADJECTIVES: &[&'static str] = &[
    "pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", 
    "quaint", "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", 
    "unsightly", "adorable", "important", "inexpensive", "cheap", "expensive", "fancy"
];

static COLOURS: &[&'static str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&'static str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

#[static_ref]
fn generator() -> &'static Mutable<SmallRng> {
    Mutable::new(SmallRng::from_entropy())
}

#[static_ref]
fn previous_id() -> &'static Mutable<ID> {
    Mutable::new(0)
}

#[static_ref]
fn selected_row() -> &'static Mutable<Option<ID>> {
    Mutable::new(None)
}

#[static_ref]
fn rows() -> &'static MutableVec<Arc<Mutable<Row>>> {
    MutableVec::new()
}

type ID = usize;

struct Row {
    id: ID,
    label: Arc<Mutable<String>>,
}

// ------ ------
//   Commands 
// ------ ------

fn create_row() -> Arc<Mutable<Row>> {
    let id = previous_id().map_mut(|id| {
        *id += 1;
        *id
    });
    let label = generator().map_mut(|generator| {
        format!(
            "{} {} {}",
            ADJECTIVES.choose(generator).unwrap(),
            COLOURS.choose(generator).unwrap(),
            NOUNS.choose(generator).unwrap(),
        )
    });
    Arc::new(Mutable::new(Row { 
        id, 
        label: Arc::new(Mutable::new(label))
    }))
}

fn create_rows(count: usize) {
    rows()
        .lock_mut()
        .replace_cloned(repeat_with(create_row).take(count).collect())
}

fn append_rows(count: usize) {
    rows()
        .lock_mut()
        .extend(repeat_with(create_row).take(count));
}

fn update_rows(step: usize) {
    let rows = rows().lock_ref();
    for position in (0..rows.len()).step_by(step) {
        rows[position].lock_ref().label.lock_mut().push_str(" !!!");
    }
}

fn clear_rows() {
    rows().lock_mut().clear()
}

fn swap_rows() {
    if rows().lock_ref().len() < 999 { return }
    rows().lock_mut().swap(1, 998)
}

fn select_row(id: ID) {
    selected_row().set_neq(Some(id))
}

fn remove_row(id: ID) {
    rows().lock_mut().retain(|row| row.lock_ref().id != id);
}

// ------ ------
//     View 
// ------ ------

fn root() -> RawEl {
    RawEl::with_tag("div")
        .attr("class", "container")
        .children(array::IntoIter::new([
            jumbotron(),
            table(),
            RawEl::with_tag("span")
                .attr("class", "preloadicon glyphicon glyphicon-remove")
                .attr("aria-hidden", "")
        ]))
}

fn jumbotron() -> RawEl {
    RawEl::with_tag("div")
        .attr("class", "jumbotron")
        .child(
            RawEl::with_tag("div")
                .attr("class", "row")
                .children(array::IntoIter::new([
                    RawEl::with_tag("div")
                        .attr("class", "col-md-6")
                        .child(
                            RawEl::with_tag("h1").child("Zoon")
                        ),
                    RawEl::with_tag("div")
                        .attr("class", "col-md-6")
                        .child(
                            action_buttons()
                        ),
                ]))
        )
}

fn action_buttons() -> RawEl {
    RawEl::with_tag("div")
        .attr("class", "row")
        .children(array::IntoIter::new([
            action_button("run", "Create 1,000 rows", || create_rows(1_000)),
            action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
            action_button("add", "Append 1,000 rows", || append_rows(1_000)),
            action_button("update", "Update every 10th row", || update_rows(10)),
            action_button("clear", "Clear", clear_rows),
            action_button("swaprows", "Swap Rows", swap_rows),
        ]))
}

fn action_button(
    id: &'static str, 
    title: &'static str, 
    on_click: fn(),
) -> RawEl {
    RawEl::with_tag("div")
        .attr("class", "col-sm-6 smallpad")
        .child(
            RawEl::with_tag("button")
                .attr("id", id)
                .attr("class", "btn btn-primary btn-block")
                .attr("type", "button")
                .event_handler(move |_: events::Click| on_click())
                .child(
                    title
                )
        )
}

fn table() -> RawEl {
    RawEl::with_tag("table")
        .attr("class", "table table-hover table-striped test-data")
        .child(
            RawEl::with_tag("tbody")
                .attr("id", "tbody")
                .children_signal_vec(
                    rows().signal_vec_cloned().map(row)
                )
        )
}

fn row(row: Arc<Mutable<Row>>) -> RawEl {
    let row = row.lock_ref();
    let id = row.id;
    RawEl::with_tag("tr")
        .attr_signal(
            "class",
            selected_row().signal().map(move |selected_id| {
                (selected_id? == id).then(|| "danger")
            })
        )
        .children(array::IntoIter::new([
            row_id(id),
            row_label(id, row.label.signal_cloned()),
            row_remove_button(id),
            RawEl::with_tag("td")
                .attr("class", "col-md-6")
        ]))
}

fn row_id(id: ID) -> RawEl {
    RawEl::with_tag("td")
        .attr("class", "col-md-1")
        .child(id)
}

fn row_label(id: ID, label: impl Signal<Item = String> + Unpin + 'static) -> RawEl {
    RawEl::with_tag("td")
        .attr("class", "col-md-4")
        .event_handler( move |_: events::Click| select_row(id))
        .child(
            RawEl::with_tag("a")
                .attr("class", "lbl")
                .child(Text::with_signal(label))
        )
}

fn row_remove_button(id: ID) -> RawEl {
    RawEl::with_tag("td")
        .attr("class", "col-md-1")
        .child(
            RawEl::with_tag("a")
                .attr("class", "remove")
                .event_handler(move |_: events::Click| remove_row(id))
                .child(
                    RawEl::with_tag("span")
                        .attr("class", "glyphicon glyphicon-remove remove")
                        .attr("aria-hidden", "true"),
                )
        )
}

// ------ ------
//     Start 
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
