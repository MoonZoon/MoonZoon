use zoon::{*, println, raw_el::{attr, tag, event_handler}};
use zoon::futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};
use rand::prelude::*;
use std::{mem, sync::Arc, iter::repeat_with};
use enclose::enc;

// ------ ------
//    Statics 
// ------ ------

static ADJECTIVES: &[&'static str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
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
fn selected_row() -> &'static Mutable<Option<Arc<Mutable<Row>>>> {
    Mutable::new(None)
}

#[static_ref]
fn rows() -> &'static MutableVec<Arc<Mutable<Row>>> {
    MutableVec::new()
}

struct Row {
    id: ID,
    label: String,
}

type ID = usize;

// ------ ------
//    Signals 
// ------ ------

fn rows_len() -> impl Signal<Item = usize> {
    rows().signal_vec_cloned().len()
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
    Arc::new(Mutable::new(Row { id, label }))
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
        rows[position].update_mut(|row| {
            row.label += " !!!"
        });
    }
}

fn clear_rows() {
    rows().lock_mut().clear()
}

fn swap_rows() {
    if rows().lock_ref().len() < 999 { return }
    rows().lock_mut().swap(1, 998)
}

fn swap_rows_special() {
    rows().lock_mut().swap(1, 3)
}

fn select_row(row: Arc<Mutable<Row>>) {
    selected_row().set(Some(row))
    // selected_row().set_neq(Some(row))
}

fn remove_row(row: Arc<Mutable<Row>>) {
    let mut rows = rows().lock_mut();
    let row_id = row.map(|row| row.id);
    let position = rows
        .iter()
        .position(|row| row.map(|row| row.id) == row_id)
        .unwrap_throw();
    rows.remove(position);
}

// ------ ------
//     View 
// ------ ------

fn root<'a>() -> RawEl<'a> {
    raw_el![
        attr("class", "container"),
        jumbotron(),
        table(),
        raw_el![
            tag("span"),
            attr("class", "preloadicon glyphicon glyphicon-remove"),
            attr("aria-hidden", ""),
        ]
    ]
}

fn jumbotron<'a>() -> RawEl<'a> {
    raw_el![
        attr("class", "jumbotron"),
        raw_el![
            attr("class", "row"),
            raw_el![
                attr("class", "col-md-6"),
                raw_el![
                    tag("h1"),
                    "Zoon",
                ]
            ],
            raw_el![
                attr("class", "col-md-6"),
                raw_el![
                    attr("class", "row"),
                    action_button("run", "Create 11 rows", || create_rows(11)),
                    action_button("run", "Create 1,000 rows", || create_rows(1_000)),
                    action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
                    action_button("add", "Append 1,000 rows", || append_rows(1_000)),
                    action_button("update", "Update every 10th row", || update_rows(10)),
                    action_button("clear", "Clear", clear_rows),
                    action_button("swaprows", "Swap Rows", swap_rows),
                    action_button("swaprows", "Swap Rows 2 and 4", swap_rows_special),
                ]
            ],
        ]
    ]
}

fn action_button<'a>(
    id: &'static str, 
    title: &'static str, 
    on_click: fn(),
) -> RawEl<'a> {
    raw_el![
        attr("class", "col-sm-6 smallpad"),
        attr("id", id),
        attr("type", "button"),
        event_handler("click", move |_| {
            println!("Clicked!");
            on_click()
        }),
        title,
    ]
}

fn table<'a>() -> RawEl<'a> {
    raw_el![
        tag("table"),
        attr("class", "table table-hover table-striped test-data"),
        raw_el![
            tag("tbody"),
            attr("id", "tbody"),
            raw_el::children_signal_vec(
                rows().signal_vec_cloned().map(row)
            )
        ]
    ]
}

fn row<'a>(row: Arc<Mutable<Row>>) -> RawEl<'a> {
    raw_el![
        tag("tr"),
        raw_el::attr_signal(
            // @TODO "danger" should be accepted without `to_string()`
            // @TODO signal_ref ?
            "class",
            selected_row().signal_cloned().map(enc!((row) move |selected_row| {
                let selected_id = selected_row.map(|selected_row| selected_row.map(|selected_row| selected_row.id));
                let row_id = row.map(|row| row.id);
                (selected_id == Some(row_id)).then(|| "danger".to_owned())
            }))
        ),
        row_id(&row),
        row_label(row.clone()),
        row_remove_button(row.clone()),
        raw_el![
            tag("td"),
            attr("class", "col-md-6"),
        ]
    ]
}

fn row_id<'a>(row: &Arc<Mutable<Row>>) -> RawEl<'a> {
    raw_el![
        tag("td"),
        attr("class", "col-md-1"),
        row.lock_ref().id
    ]
}

fn row_label<'a>(row: Arc<Mutable<Row>>) -> RawEl<'a> {
    raw_el![
        tag("td"),
        attr("class", "col-md-4"),
        event_handler("click", enc!((row) move |_| select_row(row))),
        raw_el![
            tag("a"),
            attr("class", "lbl"),
            // @TODO make label Arc + Mutable?
            row.lock_ref().label.clone(),
        ]
    ]
}

fn row_remove_button<'a>(row: Arc<Mutable<Row>>) -> RawEl<'a> {
    raw_el![
        tag("td"),
        attr("class", "col-md-1"),
        raw_el![
            tag("a"),
            attr("class", "remove"),
            event_handler("click", move |_| remove_row(row)),
            raw_el![
                tag("span"),
                attr("class", "glyphicon glyphicon-remove remove"),
                attr("aria-hidden", "true"),
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
