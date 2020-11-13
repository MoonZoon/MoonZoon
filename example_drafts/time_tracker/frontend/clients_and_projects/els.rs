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
    }

    #[el]
    fn client_name(client: Var<super::Client>) -> TextInput {
        let name = el_var(|| {
            client
                .try_map(|client| client.name.clone())
                .unwrap_or_default()
        });
        text_input![
            do_once(focus),
            text_input::on_change(|new_name| name.set(new_name)),
            on_blur(|| name.use_ref(|name| {
                super::rename_client(client, name);
            })),
            name.inner(),
        ]
    }

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
    fn project_name(client: Var<super::Client>) -> TextInput {
        let name = el_var(|| {
            project
                .try_map(|project| project.name.clone())
                .unwrap_or_default()
        });
        text_input![
            do_once(focus),
            text_input::on_change(|new_name| name.set(new_name)),
            on_blur(|| name.use_ref(|name| {
                super::rename_project(project, name);
            })),
            name.inner(),
        ]
    }
}
