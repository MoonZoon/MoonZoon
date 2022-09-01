use crate::{app::PageId, *};
use std::collections::VecDeque;
use zoon::println;

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async {
        println!("{}", routing::url());

        app::close_menu();

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return app::set_page_id(PageId::Unknown);
            }
        };

        match route {
            Route::Login => {
                if app::is_user_logged() {
                    return router().replace(Route::Root);
                }
                app::set_page_id(PageId::Login);
            }
            Route::ClientsAndProjects => {
                if not(app::is_user_logged()) {
                    return router().replace(Route::Login);
                }
                clients_and_projects_page::request_clients();
                app::set_page_id(PageId::ClientsAndProjects);
            }
            Route::TimeTracker => {
                if not(app::is_user_logged()) {
                    return router().replace(Route::Login);
                }
                time_tracker_page::request_clients();
                app::set_page_id(PageId::TimeTracker);
            }
            Route::TimeBlocks => {
                if not(app::is_user_logged()) {
                    return router().replace(Route::Login);
                }
                time_blocks_page::request_clients();
                app::set_page_id(PageId::TimeBlocks);
            }
            Route::Root => {
                app::set_page_id(PageId::Home);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
pub enum Route {
    #[route("login")]
    Login,

    #[route("clients_and_projects")]
    ClientsAndProjects,

    #[route("time_tracker")]
    TimeTracker,

    #[route("time_blocks")]
    TimeBlocks,

    #[route()]
    Root,
}
