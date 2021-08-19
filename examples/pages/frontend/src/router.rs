use crate::{
    app::{self, PageId},
    calc_page, report_page,
};
use std::collections::VecDeque;
use zoon::{println, *};

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
    Router::new(|route: Option<Route>| {
        println!("{}", routing::current_url());

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
            Route::ReportRoot => {
                if not(app::is_user_logged()) {
                    return router().replace(Route::Login);
                }
                app::set_page_id(PageId::Report);
            }
            Route::Report { frequency } => {
                if not(app::is_user_logged()) {
                    return router().replace(Route::Login);
                }
                app::set_page_id(PageId::Report);
                report_page::set_frequency(frequency);
            }
            Route::Login => {
                if app::is_user_logged() {
                    return router().replace(Route::Root);
                }
                app::set_page_id(PageId::Login);
            }
            Route::CalcRoot => {
                if let Some(expr) = calc_page::expression().get_cloned() {
                    return router().replace(expr.into_route());
                }
                app::set_page_id(PageId::Calc);
            }
            Route::Calc {
                operand_a,
                operator,
                operand_b,
            } => {
                app::set_page_id(PageId::Calc);
                calc_page::set_expression(calc_page::Expression::new(
                    operand_a, operator, operand_b,
                ));
            }
            Route::Root => {
                app::set_page_id(PageId::Home);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("report")]
    ReportRoot,
    #[route("report", frequency)]
    Report { frequency: report_page::Frequency },

    #[route("login")]
    Login,

    #[route("calc")]
    CalcRoot,
    #[route("calc", operand_a, operator, operand_b)]
    Calc {
        operand_a: f64,
        operator: String,
        operand_b: f64,
    },

    #[route()]
    Root,
}
