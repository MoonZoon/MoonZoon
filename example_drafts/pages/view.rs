use zoon::*;
use crate::app;

#[view]
pub fn view() -> Column {
    column![
        header(),
        page(),
    ]
}

#[view]
fn header() -> Row {
    row![
        link![
            link::url(app::Route::root())
            "Home"
        ],
        link![
            link::url(app::Route::admin_report(None)),
            "Report"
        ],
    ]
}

#[view]
fn page() -> El {
    let route = app::route.inner();

    match route {
        app::Route::AdminReport(_) => {
            admin_report()
        }
        app::Route::Root => {
            el![
                "welcome home!",
            ]
        }
        app::Route::Unknown => {
            el![
                "404"
            ]
        }
    }
}

#[view]
fn admin_report() -> El {
    let logged_user = app::logged_user().inner();
    let frequency = app::frequency().inner().as_str();
    let frequency_for_link = app::frequency_for_link().inner().as_str();

    el![
        row![
            format!(
                "Hello {}! This is your {} report.", 
                logged_user, frequency,
            ),
            link![
                format!{"Switch to {}", frequency_for_link}
            ]
        ]
    ]
}
