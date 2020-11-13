use zoon::*;
use crate::app;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            el![
                region::h1(),
                "Clients & Projects",
            ],
            button![
                button::on_press(super::add_client),
                "Add Client",
            ];
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
        column![
            row![
                client_name(client),
                button![
                    button::on_press(|| super::remove_client(client)),
                    "D",
                ],
            ],
            button![
                button::on_press(|| super::add_project(client)),
                "Add Project",
            ],
            project_panels(client),
        ]
    }

    #[el]
    fn client_name(client: Var<super::Client>) -> TextInput {
        let name = el_var(|| {
            client
                .try_map(|client| client.name.clone())
                .unwrap_or_default()
        });
        text_input![
            do_once(|| {
                super::added_client().map(|added_client| {
                    (added_client == Some(client)).then(focus)
                })
            }),
            text_input::on_change(|new_name| name.set(new_name)),
            on_blur(|| name.use_ref(|name| {
                super::rename_client(client, name);
            })),
            name.inner(),
        ]
    }

    // ------ Project ------

    #[el]
    fn project_panels(client: Var<super::Client>) -> Column {
        column![
            spacing(20),
            client.try_map(|client| {
                client.projects.iter().rev().map(project_panel)
            })
        ]
    }

    #[el]
    fn project_panel(project: Var<super::Project>) -> Row {
        row![
            project_name(project),
            button![
                button::on_press(|| super::remove_project(project)),
                "D",
            ],
        ]
    }

    #[el]
    fn project_name(project: Var<super::Project>) -> TextInput {
        let name = el_var(|| {
            project
                .try_map(|project| project.name.clone())
                .unwrap_or_default()
        });
        text_input![
            do_once(|| {
                super::added_project().map(|added_project| {
                    (added_project == Some(project)).then(focus)
                })
            }),
            text_input::on_change(|new_name| name.set(new_name)),
            on_blur(|| name.use_ref(|name| {
                super::rename_project(project, name);
            })),
            name.inner(),
        ]
    }
}
