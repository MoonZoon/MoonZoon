use zoon::*;
use shared::DownMsg;

pub mod els;

blocks!{
    append_blocks![els]

    #[subscription]
    fn on_route_change() {
        if let app::Route::TimeTracker = route() {
            set_clients(None);
            app::send_up_msg(false, UpMsg::GetTimeTrackerClients);
        }
    }

    #[subscription]
    fn handle_down_msg() {
        listen(|msg: Option<DownMsg>| {
            if let Some(DownMsg::TimeTrackerClients(clients)) = msg {
                set_clients(Some(clients));
                return None
            }
            msg
        })
    }

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        projects: Vec<Var<Project>>,
    }

    #[var]
    fn client_event_handler() -> VarEventHandler<Client> {
        VarEventHandler::new(|event, client| {
            if let VarAdded = event {
                clients().update_mut(|clients| {
                    if let Some(clients) = clients {
                        clients.push(client);
                    }
                });
            }
        })
    }

    #[var]
    fn clients() -> Option<Vec<Var<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::time_blocks::Client>) {
        let clients = match {
            Some(clients) => clients,
            None => return clients().set(None);
        };
        stop!{
            clients().set(Some(Vec::new()));
            for client in clients {
                let client_var = var(Client {
                    id: client.id,
                    name: client.name,
                    projects: Vec::new(),
                });
                for project in client.projects {
                    let project_var = var(Project {
                        id: project.id,
                        name: project.name,
                        active_time_entry: None,
                        time_entries: vec::new(),
                        client: client_var,
                    });  
                    for time_entry in project.time_entries {
                        let time_entry_var = var(TimeEntry {
                            id: time_entry.id,
                            name: time_entry.name,
                            started: time_entry.started,
                            stopped: time_entry.stopped,
                            project: project_var,
                        });                    
                    }                  
                }
            }
        }
    }

    // ------ Project ------

    #[derive(Debug)]
    pub struct Project {
        id: ProjectId,
        name: String,
        active_time_entry: Option<Var<TimeEntry>>,
        time_entries: Vec<Var<TimeEntry>>,
    }

    #[var]
    fn project_event_handler() -> VarEventHandler<Client> {
        VarEventHandler::new(|event, project| {
            if let VarAdded = event {
                project.client.update_mut(|client| {
                    client.projects.push(project)
                });
            }
        })
    }

    // ------ TimeEntry ------

    #[derive(Debug)]
    struct TimeEntry {
        id: TimeEntryId,
        name: String,
        started: DateTime<Local>,
        stopped: Option<DateTime<Local>>,
        project: Var<Project>,
    }

    #[var]
    fn time_entry_event_handler() -> VarEventHandler<TimeEntry> {
        VarEventHandler::new(|event, time_entry| {
            let project = || time_entry.map(|time_entry| time_entry.project);
            let stopped = time_entry.map(|time_entry| time_entry.stopped);
            match event {
                VarAdded => {
                    project().update_mut(|project| {
                        project.time_entries.push(time_entry);

                        if stopped.is_none() {
                            project.active_time_entry = Some(time_entry);
                        }
                    });
                },
                VarUpdated => (),
                VarRemoved => {
                    project().update_mut(|project| {
                        let time_entries = &mut project.time_entries;
                        let position = time_entries.iter().position(|te| te == time_entry);
                        time_entries.remove(position.unwrap());

                        if project.active_time_entry == Some(time_entry) {
                            project.active_time_entry = None;
                        }
                    });
                },
            }
        })
    }

    #[update]
    fn add_time_entry(project: Var<Project>) {
        let project_id = project.map(|project| project.id);

        let time_entry = shared::time_tracker::TimeEntry {
            id: TimeEntryId::new(),
            name: String::new(),
            started: Local::now(),
            stopped: None,
        };

        var(TimeEntry {
            id: time_entry.id,
            name: time_entry.name.clone(),
            started: time_entry.started,
            stopped: time_entry.stopped,
            project,
        });
        app::send_up_msg(true, UpMsg::AddTimeEntry(project_id, time_entry));
    }

    #[update]
    fn remove_time_entry(time_entry: Var<TimeEntry>) {
        let id = time_entry.remove().id;
        app::send_up_msg(true, UpMsg::RemoveTimeEntry(id));
    }

    #[update]
    fn rename_time_entry(time_entry: Var<TimeEntry>, name: &str) {
        time_entry.update_mut(|time_entry| {
            time_entry.name = name.to_owned();
            app::send_up_msg(true, UpMsg::RenameTimeEntry(time_entry.id, Cow::from(name)));
        });
    }

    #[update]
    fn set_time_entry_started(time_entry: Var<TimeEntry>, started: DateTime<Local>) {
        time_entry.update_mut(|time_entry| {
            time_entry.started = started;
            app::send_up_msg(true, UpMsg::SetTimeEntryStarted(time_entry.id, started));
        });
    }

    #[update]
    fn set_time_entry_stopped(time_entry: Var<TimeEntry>, stopped: DateTime<Local>) {
        time_entry.update_mut(|time_entry| {
            if time_entry.stopped.is_none() {
                time_entry.project.update_mut(|project| {
                    project.active_time_entry = None;
                })
            }
            time_entry.stopped = Some(stopped);
            app::send_up_msg(true, UpMsg::SetTimeEntryStarted(time_entry.id, stopped));
        });
    }

}
