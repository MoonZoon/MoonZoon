use educe::Educe;
use fake::{faker, Fake};
use localsearch::LocalSearch;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};
use std::borrow::Cow;
use std::iter;
use std::sync::Arc;
use zoon::{
    moonlight::Ulid,
    strum::{EnumIter, IntoEnumIterator},
    *,
};

mod ui;

const PROJECTS_PER_PAGE: usize = 7;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct CategoryFilter(Option<Category>);

impl CategoryFilter {
    pub fn iter() -> impl Iterator<Item = Self> {
        iter::once(None).chain(Category::iter().map(Some)).map(Self)
    }
}

impl<'a> IntoCowStr<'a> for CategoryFilter {
    fn into_cow_str(self) -> Cow<'a, str> {
        if let Some(category) = self.0 {
            category.into_cow_str()
        } else {
            "All Categories".into()
        }
    }
}

#[derive(Debug, Clone)]
struct Company {
    #[allow(dead_code)]
    id: Ulid,
    name: String,
    category: Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[strum(crate = "strum")]
enum Category {
    Public,
    Private,
    OnePerson,
}

impl<'a> IntoCowStr<'a> for Category {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Self::Public => "Public",
            Self::Private => "Private",
            Self::OnePerson => "One Person",
        }
        .into()
    }
}

#[static_ref]
fn indexed_companies() -> &'static Mutable<SendWrapper<LocalSearch<Arc<Company>>>> {
    fn index_all_companies() -> SendWrapper<LocalSearch<Arc<Company>>> {
        SendWrapper::new(
            LocalSearch::builder(all_companies().lock_ref().to_vec(), |company_card| {
                &company_card.name
            })
            .build(),
        )
    }
    Task::start(
        all_companies()
            .signal_vec_cloned()
            .len()
            .dedupe()
            .for_each_sync(|_| {
                indexed_companies().set(index_all_companies());
            }),
    );
    Mutable::new(index_all_companies())
}

#[static_ref]
fn all_companies() -> &'static MutableVec<Arc<Company>> {
    MutableVec::new_with_values(dummy_companies().clone())
}

#[static_ref]
fn dummy_companies() -> &'static Vec<Arc<Company>> {
    fn generate_name(rng: &mut impl Rng) -> String {
        faker::company::en::CompanyName().fake_with_rng(rng)
    }
    fn generate_category(rng: &mut impl Rng) -> Category {
        Category::iter()
            .collect::<Vec<_>>()
            .choose(rng)
            .copied()
            .unwrap_throw()
    }
    let mut rng = SmallRng::from_entropy();
    // (0..10_000)
    (0..100)
        .map(|_| {
            Arc::new(Company {
                id: Ulid::generate(),
                name: generate_name(&mut rng),
                category: generate_category(&mut rng),
            })
        })
        .collect()
}

#[static_ref]
fn search_query() -> &'static Mutable<String> {
    Mutable::default()
}

#[static_ref]
fn filtered_companies() -> &'static MutableVec<Arc<Company>> {
    let filtering = map_ref! {
        let query = search_query().signal_cloned(),
        let category = category_filter().signal().dedupe(),
        let _ = all_companies().signal_vec_cloned().len().dedupe() =>
        (query.clone(), *category)
    };
    Task::start(filtering.for_each_sync(|(query, category)| {
        let company_filter = move |company: &Arc<Company>| {
            if let Some(category) = category.0 {
                if company.category != category {
                    return false;
                }
            }
            true
        };
        let mut found_companies: Vec<_> = if query.is_empty() {
            all_companies()
                .lock_ref()
                .iter()
                .filter(|company| company_filter(company))
                .map(Arc::clone)
                .collect()
        } else {
            indexed_companies()
                .lock_ref()
                .search(&query, usize::MAX)
                .into_iter()
                .filter(|(company, _)| company_filter(company))
                .map(|(company, _)| Arc::clone(company))
                .collect()
        };
        found_companies
            .sort_unstable_by(|company_a, company_b| Ord::cmp(&company_a.name, &company_b.name));
        let mut filtered_companies = filtered_companies().lock_mut();
        filtered_companies.replace_cloned(found_companies);
    }));
    MutableVec::new()
}

#[static_ref]
fn category_filter() -> &'static Mutable<CategoryFilter> {
    Mutable::default()
}

#[static_ref]
fn current_page() -> &'static Mutable<usize> {
    let resetter = map_ref! {
        let _ = search_query().signal_ref(|_|()),
        let _ = category_filter().signal().dedupe(),
        let _ = all_companies().signal_vec_cloned().len().dedupe() =>
        ()
    };
    Task::start(resetter.for_each_sync(|_| current_page().set_neq(0)));
    Mutable::default()
}

#[derive(Educe)]
#[educe(Default(new))]
struct Store {
    #[educe(Default(expression = r#"Mutable::new(Arc::new("Hello".to_owned()))"#))]
    text_a: Mutable<Arc<String>>,
    #[educe(Default(expression = r#"Mutable::new(Arc::new("World!".to_owned()))"#))]
    text_b: Mutable<Arc<String>>,
    joined_texts: Mutable<Arc<String>>,
}

#[static_ref]
fn store() -> &'static Store {
    Store::new()
}

fn pages_count() -> impl Signal<Item = usize> {
    filtered_companies()
        .signal_vec_cloned()
        .len()
        // @TODO refactor with https://doc.rust-lang.org/std/primitive.usize.html#method.div_ceil once stable
        .map(|companies| ((companies as f64) / (PROJECTS_PER_PAGE as f64)).ceil() as usize)
}

fn filtered_companies_paginated() -> impl SignalVec<Item = Arc<Company>> {
    // @TODO is there a better solution? New `static ref` `filtered_companies_paginated`? (perhaps search for "pagination" in Dominator chat)
    filtered_companies()
        .signal_vec_cloned()
        .enumerate()
        .filter_signal_cloned(|(index, _)| {
            map_ref!{
                let index = index.signal(),
                let current_page = current_page().signal() => {
                    // @TODO refactor once `let..else` is stable (Rust 1.65?):
                    // let Some(index) = index else { return false }; 
                    if let Some(index) = index {
                        (*index >= (*current_page * PROJECTS_PER_PAGE)) && (*index < ((*current_page + 1) * PROJECTS_PER_PAGE))
                    } else {
                        false
                    }
                }
            }
        })
        .map(|(_, company)| company)
}

fn main() {
    Task::start_blocking_with_channels(
        |send_to_blocking| {
            map_ref! {
                let text_a = store().text_a.signal_cloned(),
                let text_b = store().text_b.signal_cloned() =>
                (text_a.clone(), text_b.clone())
            }
            .for_each_sync(send_to_blocking)
        },
        |from_input, _, send_to_output| {
            from_input.for_each_sync(move |(text_a, text_b)| {
                send_to_output(format!("{text_a} {text_b}"));
            })
        },
        |from_blocking| {
            from_blocking.for_each_sync(|joined_texts| {
                store().joined_texts.set(joined_texts.into());
            })
        },
    );
    start_app("app", root);
}

pub fn root() -> impl Element {
    global_styles().style_group(
        StyleGroup::new("body").style("background-color", ui::BACKGROUND_COLOR.into_cow_str()),
    );

    Column::new()
        .s(Width::default().min(550))
        .s(Padding::all(20).top(50))
        .s(Align::new().center_x())
        .s(Gap::new().y(45))
        .item(ui::AppInfo::new())
        .item(search_field())
        .item(category_filter_panel())
        .item(ui::Pagination::new(current_page().clone(), pages_count()))
        .item(results())
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
                .text_signal(search_query().signal_cloned())
                .on_change(move |new_text| search_query().set(new_text.into())),
        )
}

fn category_filter_panel() -> impl Element {
    ui::Dropdown::new(
        category_filter().signal(),
        always_vec(CategoryFilter::iter().collect()),
        |filter| category_filter().set_neq(filter),
    )
    .s(Align::new().center_x())
    .s(Width::exact(200))
}

fn results() -> impl Element {
    Column::new()
        .s(Gap::new().y(15))
        .items_signal_vec(filtered_companies_paginated().map(company_card))
}

fn company_card(company: Arc<Company>) -> impl Element {
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
                .child(company.category.into_cow_str()),
        )
}
