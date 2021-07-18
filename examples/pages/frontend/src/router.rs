use crate::{
    app::{self, PageId},
    calc_page, report_page,
};
use zoon::*;

// ------ Router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::ReportRoot) => {
            if not(app::is_user_logged()) {
                return router().replace(Route::Login);
            }
            app::set_page_id(PageId::Report);
        }
        Some(Route::Report { frequency }) => {
            if not(app::is_user_logged()) {
                return router().replace(Route::Login);
            }
            app::set_page_id(PageId::Report);
            report_page::set_frequency(frequency);
        }
        Some(Route::Login) => {
            if app::is_user_logged() {
                return router().replace(Route::Root);
            }
            app::set_page_id(PageId::Login);
        }
        Some(Route::CalcRoot) => {
            if let Some(expr) = calc_page::expression().get_cloned() {
                return router().replace(expr.into_route());
            }
            app::set_page_id(PageId::Calc);
        }
        Some(Route::Calc {
            operand_a,
            operator,
            operand_b,
        }) => {
            app::set_page_id(PageId::Calc);
            calc_page::set_expression(calc_page::Expression::new(operand_a, operator, operand_b));
        }
        Some(Route::Root) => {
            app::set_page_id(PageId::Home);
        }
        None => {
            app::set_page_id(PageId::Unknown);
        }
    })
}

// ------ Route ------

#[route]
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
