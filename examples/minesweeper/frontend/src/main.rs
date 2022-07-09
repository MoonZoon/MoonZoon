use zoon::{format, named_color::*, *};

// @TODO finish

type X = u32;
type Y = u32;

#[derive(Debug, Clone)]
struct Field {
    kind: FieldKind,
    state: Mutable<FieldState>,
}

impl Field {
    fn new_empty(mines: u16) -> Self {
        Field {
            kind: FieldKind::Empty { mines },
            state: Mutable::new(FieldState::Default),
        }
    }

    fn new_mine() -> Self {
        Field {
            kind: FieldKind::Mine,
            state: Mutable::new(FieldState::Default),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FieldKind {
    Mine,
    Empty { mines: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldState {
    Default,
    Flagged,
    Uncovered,
}

#[static_ref]
fn fields() -> &'static MutableVec<MutableVec<Field>> {
    MutableVec::new_with_values(hardcoded_fields())
}

fn hardcoded_fields() -> Vec<MutableVec<Field>> {
    vec![
        MutableVec::new_with_values(vec![
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(2),
            Field::new_mine(),
            Field::new_empty(3),
            Field::new_empty(1),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(2),
            Field::new_mine(),
            Field::new_mine(),
            Field::new_empty(1),
        ]),
    ]
}

fn flagged_count() -> impl Signal<Item = usize> {
    fields()
        .signal_vec_cloned()
        .map_signal(|fields| {
            fields
                .signal_vec_cloned()
                .filter_signal_cloned(|field| {
                    field
                        .state
                        .signal_ref(|state| matches!(state, FieldState::Flagged))
                })
                .len()
        })
        .sum()
}

fn uncover_field(field: &Field) {
    field.state.set_neq(FieldState::Uncovered)
}

fn flag_field(field: &Field) {
    let mut state = field.state.lock_mut();
    match *state {
        FieldState::Flagged => *state = FieldState::Default,
        FieldState::Default => *state = FieldState::Flagged,
        FieldState::Uncovered => (),
    }
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Spacing::new(20))
        .item(grid())
        .item(flagged_counter())
        .item(reset_button())
}

fn flagged_counter() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .child_signal(flagged_count().map(|count| format!("Flagged: {count}")))
}

fn reset_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().center_x())
        .s(Padding::new().x(20).y(10))
        .s(RoundedCorners::all(10))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_8, || RED_9)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Reset")
        .on_press(|| fields().lock_mut().replace_cloned(hardcoded_fields()))
}

fn grid() -> impl Element {
    let spacing = || Spacing::new(10);
    Column::new()
        .s(spacing())
        .items_signal_vec(
            fields()
                .signal_vec_cloned()
                .enumerate()
                .map(move |(y, fields)| {
                    Row::new().s(spacing()).items_signal_vec(
                        fields
                            .signal_vec_cloned()
                            .enumerate()
                            .map(move |(x, field)| {
                                field_button(
                                    x.get().unwrap_throw() as X,
                                    y.get().unwrap_throw() as Y,
                                    field,
                                )
                            }),
                    )
                }),
        )
}

fn field_button(x: X, y: Y, field: Field) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Button::new()
        .s(Padding::all(10))
        .s(RoundedCorners::all(10))
        .s(Height::fill())
        .s(Width::fill())
        .s(Background::new().color_signal(hovered_signal.map_bool(|| BLUE_8, || BLUE_9)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(
            El::new().s(Height::fill()).child(
                Column::new()
                    .item(El::new().child(format!("[{x}, {y}]")))
                    .item(
                        El::new()
                            .child_signal(field.state.signal_ref(|state| format!("{state:#?}"))),
                    )
                    .item(
                        El::new()
                            .s(Font::new().left())
                            .child(format!("{:#?}", field.kind)),
                    ),
            ),
        )
        // @TODO refactor together with event handler API redesign
        .update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |event: events::MouseDown| match event.button() {
                    events::MouseButton::Left => uncover_field(&field),
                    events::MouseButton::Right => flag_field(&field),
                    _ => (),
                })
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    move |event: events::ContextMenu| {
                        event.prevent_default();
                    },
                )
        })
}

fn main() {
    start_app("app", root);
}
