use zoon::*;
use crate::app;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            el![
                region::h1(),
                "Time Blocks",
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
                clients.unwrap_or_default().iter().rev().map(client_panel)
            }),
        ]
    }

    #[el]
    fn client_panel(client: Var<super::Client>) -> Column {
        let statistics = client.try_map(|client| client.statistics).expect("client statistics");
        column![
            row![
                el![client],
                statistics(statistics),
            ],
            button![
                button::on_press(|| super::add_time_block(client)),
                "Add Time Block",
            ],
            time_block_panels(client),
        ]
    }

    #[el]
    fn statistics(statistics: Var<super::Statistics>) -> Row {
        let statistics = statistics.try_inner().expect("statistics data");
        let format = |value: f64| format!("{:.1}", value);
        row![
            column![
                row!["Blocked", format(statistics.blocked)],
                row!["Unpaid", format(statistics.unpaid)],
                row!["Paid", format(statistics.paid)],
            ],
            column![
                row!["Tracked", format(statistics.tracked)],
                row!["To Block", format(statistics.to_block)],
            ],
        ]
    }

    // ------ TimeBlock ------

    // #[el]
    // fn time_block_panels(client: Var<super::Client>) -> Column {
    //     column![
    //         spacing(20),
    //         client.try_map(|client| {
    //             client.projects.iter().rev().map(project_panel)
    //         })
    //     ]
    // }

    // #[el]
    // fn project_panel(project: Var<super::Project>) -> Row {
    //     row![
    //         project_name(project),
    //         button![
    //             button::on_press(|| super::remove_project(project)),
    //             "D",
    //         ],
    //     ]
    // }

    // #[el]
    // fn project_name(project: Var<super::Project>) -> TextInput {
    //     let name = el_var(|| {
    //         project
    //             .try_map(|project| project.name.clone())
    //             .unwrap_or_default()
    //     });
    //     text_input![
    //         do_once(|| {
    //             super::added_project().map(|added_project| {
    //                 (added_project == Some(project)).then(focus)
    //             })
    //         }),
    //         text_input::on_change(|new_name| name.set(new_name)),
    //         on_blur(|| name.use_ref(|name| {
    //             super::rename_project(project, name);
    //         })),
    //         name.inner(),
    //     ]
    // }

}
