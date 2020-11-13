use zoon::*;
use crate::app;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            page_title();
            add_client_button();
            client_panels();
        ]
    }

    #[el]
    fn page_title() -> El {
        el![
            region::h1(),
            "Clients & Projects",
        ]
    }

    #[el]
    fn add_client_button() -> Button {
        button![
            button::on_press(super::add_client),
            "Add Client",
        ]
    }

    #[el]
    fn client_panels() -> Column {
        
    }

}
