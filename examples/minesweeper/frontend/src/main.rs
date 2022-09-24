kuse rand::thread_rng;
use zoon::{named_color::*, *};

#[derive(Clone, Debug, Default)]
struct Field {
    mine: FieldMine,
    state: Mutable<FieldState>,
}

#[derive(Clone, Debug)]
enum FieldMine {
    Mine,
    Empty(u8),
}
impl Default for FieldMine {
    fn default() -> Self {
        Self::Empty(0)
    }
}
#[derive(Clone, Debug, Default)]
enum FieldState {
    #[default]
    Covered,
    Uncovered,
    Flagged,
}

fn right_click(field: &Field) {
    let mut state = field.state.lock_mut();
    match *state {
        FieldState::Covered => *state = FieldState::Flagged,
        FieldState::Flagged => *state = FieldState::Covered,
        FieldState::Uncovered => (),
    }
}

fn left_click(field: &Field, y: usize, x: usize) {
    if !game_state().get() {
        start_game(y, x)
    }
    field.state.set(FieldState::Uncovered);
    match &field.mine {
        FieldMine::Empty(m) => {
            if m == &0 {
                open_neighbours(neighbours(y, x))
            }
        }
        FieldMine::Mine => {
            game_state().set(false);
            reset();
        }
    }
}

fn neighbours(y: usize, x: usize) -> Vec<(usize, usize)> {
    let y_start = if y > 0 { y - 1 } else { y };
    let y_end = if y < 7 { y + 1 } else { y };
    let x_start = if x > 0 { x - 1 } else { x };
    let x_end = if x < 7 { x + 1 } else { x };
    let mut n = vec![];
    for i in y_start..y_end + 1 {
        for j in x_start..x_end + 1 {
            n.push((i, j));
        }
    }
    let i = n
        .iter()
        .enumerate()
        .find(|(_, field)| field.0 == y && field.1 == x);
    n.remove(i.unwrap().0);
    n
}

fn reset() {
    for y in 0..8 {
        for x in 0..8 {
            if let Some(x_fields) = fields().lock_ref().get(y) {
                x_fields.lock_mut().set_cloned(x, Field::default());
            }
        }
    }
    game_state().set(false)
}

fn get_field(n: (usize, usize)) {
    if let Some(x_fields) = fields().lock_ref().get(n.0) {
        if let Some(field) = x_fields.lock_ref().get(n.1) {
            open_field(field, n)
        }
    }
}

fn open_field(field: &Field, n: (usize, usize)) {
    match field.mine {
        FieldMine::Mine => (),
        FieldMine::Empty(m) => {
            let state = field.state.get_cloned();
            match state {
                FieldState::Covered => {
                    field.state.set(FieldState::Uncovered);
                    if m == 0 {
                        open_neighbours(neighbours(n.0, n.1));
                    }
                }
                _ => (),
            }
        }
    }
}
// open recursive neighbour fields
fn open_neighbours(neigh: Vec<(usize, usize)>) {
    for n in neigh {
        get_field(n);
    }
}
fn start_game(y: usize, x: usize) {
    let mut neighbours = neighbours(y, x);
    // created mines should not be on clicked field or its neighbours.
    neighbours.push((y, x));
    let mut mine = 0;
    while mine < 5 {
        use rand::Rng;
        let mut rng = thread_rng();
        let y_index: usize = rng.gen_range(0..8);
        let x_index: usize = rng.gen_range(0..8);
        if !neighbours
            .iter()
            .any(|coordinates| y_index == coordinates.0 && x_index == coordinates.1)
        {
            mine += 1;
            if let Some(x_fields) = fields().lock_ref().get(y_index) {
                //if let Some(field) = x_fields.lock_ref().get(x_index) {}
                x_fields.lock_mut().set_cloned(
                    x_index,
                    Field {
                        mine: FieldMine::Mine,
                        state: Mutable::new(FieldState::Covered),
                    },
                );
            }
            // dont create another mine at same field.
            neighbours.push((y_index, x_index));
            increase_emtpy(y_index, x_index);
        }
    }
    game_state().set(true)
}

/// When game start, it would increase m of empty(m) field.mine of clicked field's neighbour
fn increase_emtpy(y: usize, x: usize) {
    for f in neighbours(y, x) {
        if let Some(x_fields) = fields().lock_ref().get(f.0) {
            let mut i = false;
            let mut new_field = Field::default();
            if let Some(field) = x_fields.lock_ref().get(f.1) {
                match field.mine {
                    FieldMine::Empty(m) => {
                        i = true;
                        new_field = Field {
                            mine: FieldMine::Empty(m + 1),
                            state: field.state.clone(),
                        };
                    }
                    _ => {}
                }
            }
            if i {
                x_fields.lock_mut().set_cloned(f.1, new_field);
            }
        }
    }
}

#[static_ref]
fn game_state() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn fields() -> &'static MutableVec<MutableVec<Field>> {
    let fields = MutableVec::new();
    for _y in 0..8 {
        let x_fields = MutableVec::new();
        for _x in 0..8 {
            x_fields.lock_mut().push_cloned(Field::default())
        }
        fields.lock_mut().push_cloned(x_fields)
    }
    fields
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Spacing::new(2))
        .items_signal_vec(
            fields()
                .signal_vec_cloned()
                .enumerate()
                .map(|(y, fields)| grid(fields, y.get().unwrap_throw())),
        )
        .item(reset_button())
}

fn grid(fields: MutableVec<Field>, y: usize) -> impl Element {
    Row::new().s(Spacing::new(3)).items_signal_vec(
        fields
            .signal_vec_cloned()
            .enumerate()
            .map(move |(x, field)| mine(field, x.get().unwrap_throw(), y)),
    )
}

fn mine(field: Field, x: usize, y: usize) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let mine = field.mine.clone();
    Button::new()
        .s(RoundedCorners::all(3))
        .s(Width::exact(50))
        .s(Height::exact(50))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_3, || GRAY_6)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(
            Label::new()
                .label_signal(field.state.signal_ref(move |state| match *state {
                    FieldState::Covered => "".into_cow_str(),
                    FieldState::Flagged => "F".into_cow_str(),
                    FieldState::Uncovered => match mine {
                        FieldMine::Mine => "M".into_cow_str(),
                        FieldMine::Empty(m) => m.into_cow_str(),
                    },
                }))
                .s(Align::center()),
        )
        .update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |event: events::MouseDown| match event.button() {
                    events::MouseButton::Left => left_click(&field, y, x),
                    events::MouseButton::Right => right_click(&field),
                    _ => (),
                })
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::ContextMenu| {
                        event.prevent_default();
                    },
                )
        })
}

fn reset_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_3, || RED_6)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Reset")
        .on_click(reset)
}

fn main() {
    start_app("app", root);
}

