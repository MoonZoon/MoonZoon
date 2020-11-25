use zoon::{*, html::{attr, tag, event_handler}};
use rand::prelude::*;
use std::iter;

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

type ID = usize;

blocks!{

    #[var]
    fn generator() -> SmallRng {
        SmallRng::from_entropy()
    }

    #[var]
    fn previous_id() -> ID {
        0
    }

    struct Row {
        id: ID,
        label: String,
    }

    #[var]
    fn selected_row() -> Option<Var<Row>> {
        None
    }

    #[var]
    fn rows() -> Vec<VarH<Row>> {
        Vec::new()
    }

    #[cache]
    fn rows_len() -> usize {
        rows().map(Vec::len)
    }

    fn create_row() -> VarH<Row> {
        let id = previous_id().map_mut(|id| {
            *id += 1;
            id
        });
        let label = format!(
            "{} {} {}",
            ADJECTIVES.choose(generator).unwrap(),
            COLOURS.choose(generator).unwrap(),
            NOUNS.choose(generator).unwrap(),
        );
        var(Row { id, label })
    }

    #[update]
    fn create_rows(count: usize) {
        rows.update_mut(|rows| {
            *rows = (0..count).map(|_| create_row()).collect();
        });
    }

    #[update]
    fn append_rows(count: usize) {
        rows.update_mut(|rows| {
            rows.append(&mut (0..count).map(|_| create_row()).collect());
        });
    }

    #[update]
    fn update_rows(step: usize) {
        let len = rows_len().inner();
        rows().use_ref(|rows| {
            stop![
                for position in (0..len).step_by(step) {
                    rows[position].update_mut(|row| row.label += " !!!");
                }
            ]
        })
    }

    #[update]
    fn clear_rows() {
        rows().update_mut(|rows| {
            rows.clear();
        })
        selected_row().set(None);
    }

    #[update]
    fn swap_rows() {
        if rows_len().inner() < 999 { return; }
        rows().update_mut(|rows| {
            rows.swap(1, 998)
        });
    }

    #[update]
    fn select_row(row: Var<Row>) {
        let old_selected = selected_row().map_mut(|selected_row| {
            selected_row.replace(row)
        });
        row.mark_updated();
        if let Some(old_selected) = old_selected {
            old_selected.mark_updated();
        }
    }

    #[update]
    fn remove(row: Var<Row>) {
        rows().update_mut(|rows| {
            let position = rows.iter_vars().position(|r| r == row).unwrap();
            rows.remove(position);
        });
        if matches!(selected_row().inner(), Some(selected_row) if selected_row == row) {
            selected_row().set(None)
        }
    }

    #[el]
    fn root() -> RawEl {
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

    #[el]
    fn jumbotron() -> RawEl {
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
                        action_button("run", "Create 1,000 rows", || create_rows(1_000)),
                        action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
                        action_button("add", "Append 1,000 rows", || append_rows(1_000)),
                        action_button("update", "Update every 10th row", || update_rows(10)),
                        action_button("clear", "Clear", clear_rows),
                        action_button("swaprows", "Swap Rows", swap_rows),
                    ]
                ],
            ]
        ]
    }

    fn action_button(
        id: &'static str, 
        title: &'static str, 
        on_click: fn(),
    ) -> RawEl {
        raw_el![
            attr("class", "col-sm-6 smallpad"),
            attr("id", id),
            attr("type", "button"),
            event_handler("click", |_| on_click())
            title,
        ]
    }

    #[el]
    fn table() -> RawEl {
        let rows = rows().map(|rows| rows.iter_vars().map(row));
        raw_el![
            tag("table"),
            attr("class", "table table-hover table-striped test-data"),
            raw_el![
                tag("tbody"),
                attr("id", "tbody"),
                rows,
            ]
        ]
    }

    #[el]
    fn row(row: Var<Row>) -> RawEl {
        let selected_row = selected_row().unwatch().inner();
        let is_selected = selected_row == Some(row);
        raw_el![
            tag("tr"),
            is_selected.then(|| attr("class", "danger")),
            row_id(row),
            row_label(row),
            row_remove_button(row),
            raw_el![
                tag("td"),
                attr("class", "col-md-6"),
            ]
        ]
    }

    #[el]
    fn row_id(row: Var<Row>) -> RawEl {
        let id = row.map(|row| row.id);
        raw_el![
            tag("td"),
            attr("class", "col-md-1"),
            id
        ]
    }

    #[el]
    fn row_label(row: Var<Row>) -> RawEl {
        let label = row.map(|row| row.label.clone());
        raw_el![
            tag("td"),
            attr("class", "col-md-4"),
            event_handler("click", |_| select_row(row)),
            raw_el![
                tag("a"),
                attr("class", "lbl"),
                label,
            ]
        ]
    }

    #[el]
    fn row_remove_button(row: Var<Row>) -> RawEl {
        row.unwatch();
        raw_el![
            tag("td"),
            attr("class", "col-md-1"),
            raw_el![
                tag("a"),
                attr("class", "remove"),
                event_handler("click", |_| remove_row(row)),
                raw_el![
                    tag("span"),
                    attr("class", "glyphicon glyphicon-remove remove"),
                    attr("aria-hidden", "true"),
                ]
            ]
        ]
    }

}

fn main() {
    start!("main")
}
