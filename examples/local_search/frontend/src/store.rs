use crate::{DEFAULT_GENERATE_COMPANIES_COUNT, PROJECTS_PER_PAGE};
use localsearch::LocalSearch;
use std::sync::Arc;
use zoon::{
    strum::{EnumIter, IntoStaticStr},
    *,
};

#[static_ref]
pub fn store() -> &'static Store {
    create_triggers();
    Store::new()
}

#[derive(Debug, Clone)]
pub struct Company {
    pub name: Arc<String>,
    pub category: Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr)]
#[strum(crate = "strum")]
pub enum Category {
    Public,
    Private,
    #[strum(serialize = "One Person")]
    OnePerson,
}

#[derive(Educe)]
#[educe(Default(new))]
pub struct Store {
    pub indexed_companies: Mutable<Option<Arc<LocalSearch<Company>>>>,
    pub all_companies: Mutable<Arc<Vec<Company>>>,
    pub search_query: Mutable<Arc<String>>,
    pub category_filter: Mutable<Option<Category>>,
    pub current_page: Mutable<usize>,
    pub generated_company_count: Mutable<usize>,
    #[educe(Default(expression = Mutable::new(DEFAULT_GENERATE_COMPANIES_COUNT.to_string())))]
    pub generate_companies_input_text: Mutable<String>,
    pub generate_companies_button_disabled: Mutable<bool>,
    pub generate_companies_time: Mutable<Option<f64>>,
    pub index_companies_time: Mutable<Option<f64>>,
    pub search_time: Mutable<Option<f64>>,
    pub search_time_history: MutableVec<f64>,
    // -- caches --
    pub filtered_companies: Mutable<Vec<Company>>,
    pub page_count: Mutable<usize>,
    pub current_page_companies: Mutable<Vec<Company>>,
}

fn create_triggers() {
    set_search_time_history_on_search_time_change();
    set_current_page_on_filtered_companies_change();
    set_current_page_companies_on_current_page_or_filtered_companies_change();
    set_indexed_companies_and_search_time_history_on_all_companies_change();
    set_filtered_companies_and_page_count_on_query_or_filter_or_indexed_companies_change();
}

fn set_search_time_history_on_search_time_change() {
    Task::start(async {
        store()
            .search_time
            .signal()
            .for_each_sync(|search_time| {
                if let Some(search_time) = search_time {
                    store().search_time_history.lock_mut().push(search_time);
                }
            })
            .await
    });
}

fn set_current_page_on_filtered_companies_change() {
    Task::start(async {
        store()
            .filtered_companies
            .signal_ref(|_| ())
            .for_each_sync(|_| {
                store().current_page.set(0);
            })
            .await
    });
}

fn set_current_page_companies_on_current_page_or_filtered_companies_change() {
    Task::start(async {
        store()
            .current_page
            .signal()
            .for_each_sync(|current_page| {
                let filtered_companies = store().filtered_companies.lock_ref();
                let index_from = current_page * PROJECTS_PER_PAGE;
                let index_to =
                    ((current_page + 1) * PROJECTS_PER_PAGE).min(filtered_companies.len());
                *store().current_page_companies.lock_mut() =
                    filtered_companies[index_from..index_to].to_vec();
            })
            .await
    });
}

fn set_indexed_companies_and_search_time_history_on_all_companies_change() {
    Task::start_blocking_with_tasks(
        |send_to_blocking| async move {
            store()
                .all_companies
                .signal_cloned()
                .for_each_sync(move |all_companies| {
                    store().index_companies_time.set(None);
                    let start_time = performance().now();
                    send_to_blocking((start_time, all_companies));
                })
                .await
        },
        |from_input, _, send_to_output| {
            from_input.for_each_sync(move |(start_time, all_companies)| {
                // @TODO refactor the expression below once `Arc::unwrap_or_clone` is stable
                let all_companies = Arc::try_unwrap(all_companies)
                    .unwrap_or_else(|all_companies| (*all_companies).clone());
                let indexed_companies =
                    LocalSearch::builder(all_companies, |company_card| &company_card.name).build();
                send_to_output((start_time, indexed_companies));
            })
        },
        |from_blocking| {
            from_blocking.for_each_sync(move |(start_time, indexed_companies)| {
                store()
                    .index_companies_time
                    .set(Some(performance().now() - start_time));

                store()
                    .indexed_companies
                    .set(Some(Arc::new(indexed_companies)));
                store().search_time_history.lock_mut().clear();
            })
        },
    );
}

fn set_filtered_companies_and_page_count_on_query_or_filter_or_indexed_companies_change() {
    Task::start_blocking_with_tasks(
        |send_to_blocking| async move {
            map_ref! {
                let query = store().search_query.signal_cloned(),
                let category = store().category_filter.signal(),
                let indexed_companies = store().indexed_companies.signal_cloned() =>
                (query.clone(), *category, indexed_companies.clone())
            }
            .for_each_sync(|(query, category, indexed_companies)| {
                store().search_time.set(None);
                let start_time = performance().now();
                send_to_blocking((
                    start_time,
                    query,
                    category,
                    indexed_companies,
                    store().all_companies.lock_ref().clone(),
                ))
            })
            .await
        },
        |from_input, _, send_to_output| {
            let task = Mutable::new(None);
            from_input.for_each_sync(
                move |(start_time, query, category, indexed_companies, all_companies)| {
                    task.set(Some(Task::start_droppable(clone!((send_to_output) async move {
                        let company_filter = move |company: &&Company| {
                            not(matches!(category, Some(category) if company.category != category))
                        };
                        let found_companies: Vec<_> = if query.is_empty() {
                            all_companies
                                .iter()
                                .filter(company_filter)
                                .map(Clone::clone)
                                .collect()
                        } else {
                            if let Some(indexed_companies) = indexed_companies.as_ref() {
                                let companies = indexed_companies.search(&query, usize::MAX);
                                Task::next_micro_tick().await;
                                companies.into_iter()
                                    .map(|(company, _)| company)
                                    .filter(company_filter)
                                    .map(Clone::clone)
                                    .collect()
                            } else {
                                Vec::new()
                            }
                        };
                        Task::next_micro_tick().await;
                        send_to_output((start_time, found_companies));
                    }))));
                },
            )
        },
        |from_blocking| {
            from_blocking.for_each_sync(move |(start_time, found_companies)| {
                store()
                    .search_time
                    .set(Some(performance().now() - start_time));

                store().page_count.set_neq(
                    // @TODO refactor with https://doc.rust-lang.org/std/primitive.usize.html#method.div_ceil once stable
                    ((found_companies.len() as f64) / (PROJECTS_PER_PAGE as f64)).ceil() as usize,
                );

                *store().filtered_companies.lock_mut() = found_companies;
            })
        },
    );
}
