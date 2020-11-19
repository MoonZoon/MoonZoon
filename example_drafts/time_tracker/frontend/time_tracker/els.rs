use zoon::*;
use crate::app;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            el![
                region::h1(),
                "Time Tracker",
            ],
            client_panels();
        ]
    }

    // ------ Client ------

    #[el]
    fn client_panels() -> Column {
        let clients = super::clients();
        column![
            spacing(30),
            clients.map(|clients| {
                clients.map(|clients| clients.iter().rev().map(client_panel))
            }),
        ]
    }

    #[el]
    fn client_panel(client: Var<super::Client>) -> Column {
        column![
            el![client.map(|client| client.name.clone())],
            project_panels(client),
        ]
    }

    // ------ Project ------

    #[el]
    fn project_panels(client: Var<super::Client>) -> Column {
        column![
            spacing(20),
            client.map(|client| {
                client.projects.iter().rev().map(project_panel)
            }),
        ]
    }

    #[el]
    fn project_panel(project: Var<super::Project>) -> Column {
        column![
            row![
                el![project.map(|project| project.name.clone())],
                start_stop_button(project),
            ],
            button![
                button::on_press(|| super::add_time_block(client)),
                "Add Time Block",
            ],
            time_block_panels(client),
        ]
    }

    #[el]
    fn start_stop_button(project: Var<super::Project>) -> Button {
        if let Some(time_entry) = project.map(|project| project.active_time_entry) {
            button![
                button::on_press(|| super::set_time_entry_stopped(time_entry, Local::now())),
                "Stop",
            ]
        } else {
            button![
                button::on_press(|| super::add_time_entry(project)),
                "Start",
            ]
        }
    }

    // ------ TimeEntry ------

    #[el]
    fn client_panels() -> Column {
        let clients = super::clients();
        column![
            spacing(30),
            clients.map(|clients| {
                clients.map(|clients| clients.iter().rev().map(client_panel))
            }),
        ]
    }

    #[el]
    fn client_panel(client: Var<super::Client>) -> Column {
        let statistics = client.map(|client| client.statistics);
        column![
            row![
                el![client.map(|client| client.name.clone())],
                statistics(statistics),
            ],
            button![
                button::on_press(|| super::add_time_block(client)),
                "Add Time Block",
            ],
            time_block_panels(client),
        ]
    }
    
}
