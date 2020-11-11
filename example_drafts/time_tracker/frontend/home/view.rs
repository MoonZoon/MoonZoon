use zoon::*;
use crate::app;

zoons!{
    
    #[view]
    fn page() -> Column {
        column![
            el![
                "Time Tracker",
            ],
            link![
                link::url(app::Route::time_tracker()),
                "Go to Time Tracker",
            ],
        ]
    }

}
