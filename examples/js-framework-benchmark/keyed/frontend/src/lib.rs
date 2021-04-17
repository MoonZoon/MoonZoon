// #![no_std]

use zoon::{*, raw_el::{attr, tag, event_handler}};
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

    #[s_var]
    fn generator() -> SVar<SmallRng> {
        SmallRng::from_entropy()
    }

    #[s_var]
    fn previous_id() -> SVar<ID> {
        0
    }

    struct Row {
        id: ID,
        label: String,
    }

    #[s_var]
    fn selected_row() -> SVar<Option<Var<Row>>> {
        None
    }

    #[s_var]
    fn rows() -> SVar<Vec<VarC<Row>>> {
        Vec::new()
    }

    #[cache]
    fn rows_len() -> Cache<usize> {
        rows().map(Vec::len)
    }

    fn create_row() -> VarC<Row> {
        let id = previous_id().map_mut(|id| {
            *id += 1;
            id
        });
        let label = generator().map_mut(|generator| {
            format!(
                "{} {} {}",
                ADJECTIVES.choose(generator).unwrap(),
                COLOURS.choose(generator).unwrap(),
                NOUNS.choose(generator).unwrap(),
            )
        });
        new_var_c(Row { id, label })
    }

    #[update]
    fn create_rows(count: usize) {
        rows().update_mut(|rows| {
            *rows = (0..count).map(|_| create_row()).collect();
        });
    }

    #[update]
    fn append_rows(count: usize) {
        rows().update_mut(|rows| {
            rows.append(&mut (0..count).map(|_| create_row()).collect());
        });
    }

    #[update]
    fn update_rows(step: usize) {
        let len = rows_len().inner();
        rows().use_ref(|rows| {
            // stop![
                for position in (0..len).step_by(step) {
                    rows[position].update_mut(|row| row.label += " !!!");
                }
            // ]
        })
    }

    #[update]
    fn clear_rows() {
        rows().update_mut(|rows| {
            rows.clear();
        })
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
            old_selected.try_mark_updated();
        }
    }

    #[update]
    fn remove(row: Var<Row>) {
        rows().update_mut(|rows| {
            let position = rows.iter_vars().position(|r| r == row).unwrap();
            rows.remove(position);
        });
    }

    #[cmp]
    fn root() -> Cmp {
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

    #[cmp]
    fn jumbotron() -> Cmp {
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

    fn action_button<'a>(
        id: &'static str, 
        title: &'static str, 
        on_click: fn(),
    ) -> RawEl<'a> {
        raw_el![
            attr("class", "col-sm-6 smallpad"),
            attr("id", id),
            attr("type", "button"),
            event_handler("click", |_| on_click()),
            title,
        ]
    }

    #[cmp]
    fn table() -> Cmp {
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

    #[cmp(row)]
    fn row(row: Var<Row>) -> Cmp {
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

    #[cmp]
    fn row_id(row: Var<Row>) -> Cmp {
        let id = row.map(|row| row.id);
        raw_el![
            tag("td"),
            attr("class", "col-md-1"),
            id
        ]
    }

    #[cmp]
    fn row_label(row: Var<Row>) -> Cmp {
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

    #[cmp]
    fn row_remove_button(row: Var<Row>) -> Cmp {
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

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
