use fake::{faker, Fake};
use num_format::{Locale, ToFormattedString};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use std::iter;
use std::sync::Arc;
use zoon::{strum::IntoEnumIterator, *};

mod ui;

mod store;
use store::*;

const PROJECTS_PER_PAGE: usize = 7;
const BACKGROUND_COLOR: HSLuv = hsluv!(0, 0, 80);
const DEFAULT_GENERATE_COMPANIES_COUNT: usize = 1_000;

fn main() {
    STORE;
    start_app("app", root);
    generate_companies(DEFAULT_GENERATE_COMPANIES_COUNT);
}

fn generate_companies(count: usize) {
    enum BlockingOutputMsg {
        CompanyIndex(usize),
        Companies(Vec<Company>),
    }

    STORE.generate_companies_button_disabled.set(true);
    STORE.generate_companies_time.set(None);
    STORE.index_companies_time.set(None);
    STORE.search_time.set(None);
    STORE.search_time_history.lock_mut().clear();

    let start_time = performance().now();

    Task::start_blocking_with_output_task(
        move |_, send_to_output| async move {
            let mut rng = SmallRng::from_entropy();
            let categories = Category::iter().collect::<Vec<_>>();

            let mut companies = Vec::with_capacity(count);
            for index in 0..count {
                companies.push(Company {
                    name: faker::company::en::CompanyName().fake_with_rng(&mut rng),
                    category: categories.choose(&mut rng).copied().unwrap_throw(),
                });
                send_to_output(BlockingOutputMsg::CompanyIndex(index));
            }
            companies.sort_unstable_by(|company_a, company_b| {
                Ord::cmp(&company_a.name, &company_b.name)
            });
            send_to_output(BlockingOutputMsg::Companies(companies))
        },
        move |from_blocking| {
            from_blocking.for_each_sync(move |msg| match msg {
                BlockingOutputMsg::CompanyIndex(index) => {
                    STORE.generated_company_count.set(index + 1);
                }
                BlockingOutputMsg::Companies(companies) => {
                    STORE
                        .generate_companies_time
                        .set(Some(performance().now() - start_time));
                    *STORE.all_companies.lock_mut() = Arc::new(companies);
                    STORE.generate_companies_button_disabled.set(false);
                }
            })
        },
    );
}

fn root() -> impl Element {
    global_styles().style_group(
        StyleGroup::new("body").style("background-color", BACKGROUND_COLOR.into_cow_str()),
    );

    Column::new()
        .s(Width::default().min(600))
        .s(Padding::all(20).top(50))
        .s(Align::new().center_x())
        .s(Gap::new().y(45))
        .item(generate_companies_panel())
        .item(search_field())
        .item(category_filter_panel())
        .item(ui::Pagination::new(
            STORE.current_page.clone(),
            STORE.page_count.signal(),
        ))
        .item(results())
}

fn generate_companies_panel() -> impl Element {
    Column::new()
        .s(Gap::new().y(20))
        .item(
            Row::new()
                .s(Gap::new().x(25))
                .item(generate_companies_button())
                .item(generate_companies_input()),
        )
        .item(companies_generated_and_indexed_text())
}

fn generate_companies_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let disabled_color = BACKGROUND_COLOR.update_l(|l| l - 30.);
    Button::new()
        .s(Outline::with_signal(
            STORE
                .generate_companies_button_disabled
                .signal_ref(move |disabled| {
                    let outline = Outline::inner().width(2);
                    if *disabled {
                        outline.color(disabled_color)
                    } else {
                        outline
                    }
                }),
        ))
        .s(Font::new().color_signal(
            STORE
                .generate_companies_button_disabled
                .signal()
                .map_true(move || disabled_color),
        ))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all(3))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().x(6).y(6)]))
        .s(Background::new().color_signal(map_ref! {
            let hovered = hovered_signal,
            let disabled = STORE.generate_companies_button_disabled.signal() =>
            if *hovered && not(disabled) {
                BACKGROUND_COLOR.update_l(|l| l + 10.)
            } else {
                BACKGROUND_COLOR
            }
        }))
        .s(Cursor::with_signal(
            STORE
                .generate_companies_button_disabled
                .signal()
                .map_bool(|| CursorIcon::Default, || CursorIcon::Pointer),
        ))
        .label("Generate companies")
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(|| {
            if not(STORE.generate_companies_button_disabled.get()) {
                if let Ok(count) = STORE
                    .generate_companies_input_text
                    .lock_ref()
                    .parse::<usize>()
                {
                    generate_companies(count);
                }
            }
        })
}

fn generate_companies_input() -> impl Element {
    TextInput::new()
        .s(Transform::new().move_down(3))
        .s(Background::new().color(hsluv!(0, 0, 0, 0)))
        .s(Shadows::new([Shadow::new().inner().x(3).y(3)]))
        .s(RoundedCorners::all(3))
        .s(Outline::inner().width(2))
        .s(Padding::new().x(10).y(10))
        .s(Width::exact(100))
        .placeholder(Placeholder::new("100"))
        .label_hidden("generate companies count")
        .text_signal(STORE.generate_companies_input_text.signal_cloned())
        .on_change(|text| STORE.generate_companies_input_text.set(text))
}

fn companies_generated_and_indexed_text() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(
                    STORE
                        .generated_company_count
                        .signal()
                        .map(|count| count.to_formatted_string(&Locale::en)),
                ),
        )
        .content(" companies generated & sorted in ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(STORE.generate_companies_time.signal_ref(|time| {
                    if let Some(time) = time {
                        format!("{time:.2}").into_cow_str()
                    } else {
                        "...".into_cow_str()
                    }
                })),
        )
        .content(" ms")
        .content(", indexed in ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(STORE.index_companies_time.signal_ref(|time| {
                    if let Some(time) = time {
                        format!("{time:.2}").into_cow_str()
                    } else {
                        "...".into_cow_str()
                    }
                })),
        )
        .content(" ms")
}

fn search_field() -> impl Element {
    let id = "search_field";
    Column::new()
        .s(Gap::new().y(10))
        .item(Label::new().for_input(id).label("Search Query"))
        .item(
            TextInput::new()
                .id(id)
                .s(Background::new().color(hsluv!(0, 0, 0, 0)))
                .s(Shadows::new([Shadow::new().inner().x(3).y(3)]))
                .s(RoundedCorners::all(3))
                .s(Outline::inner().width(2))
                .s(Padding::new().x(15).y(10))
                .placeholder(Placeholder::new("Company Name"))
                .text_signal(STORE.search_query.signal_cloned())
                .on_change(move |new_text| STORE.search_query.set(new_text.into())),
        )
        .item(companies_found_info())
}

fn companies_found_info() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(
                    STORE
                        .filtered_companies
                        .signal_ref(|filtered_companies| filtered_companies.len())
                        .map(|count| count.to_formatted_string(&Locale::en)),
                ),
        )
        .content(" companies found in ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(STORE.search_time.signal_ref(|time| {
                    if let Some(time) = time {
                        format!("{time:.2}").into_cow_str()
                    } else {
                        "...".into_cow_str()
                    }
                })),
        )
        .content(" ms")
        .content(" (Average: ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(
                    STORE
                        .search_time_history
                        .signal_vec()
                        .to_signal_map(|history| {
                            if history.is_empty() {
                                "...".into_cow_str()
                            } else {
                                let average = history.iter().sum::<f64>() / (history.len() as f64);
                                format!("{average:.2}").into_cow_str()
                            }
                        }),
                ),
        )
        .content(" ms)")
}

fn category_filter_panel() -> impl Element {
    ui::Dropdown::new(
        STORE.category_filter.signal(),
        always_vec(iter::once(None).chain(Category::iter().map(Some)).collect()),
        |filter| filter.map(Into::into).unwrap_or("All Categories"),
        |filter| STORE.category_filter.set_neq(*filter),
    )
    .s(Align::new().center_x())
    .s(Width::exact(200))
}

fn results() -> impl Element {
    Column::new().s(Gap::new().y(15)).items_signal_vec(
        STORE
            .current_page_companies
            .signal_cloned()
            .to_signal_vec()
            .map(company_card),
    )
}

fn company_card(company: Company) -> impl Element {
    Row::new()
        .s(Padding::new().x(15).y(10))
        .s(Gap::new().x(10))
        .s(Outline::inner().width(2))
        .s(RoundedCorners::all(3))
        .item(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child(company.name.clone()),
        )
        .item(
            El::new()
                .s(Align::new().right())
                .child(<&str>::from(company.category)),
        )
}
