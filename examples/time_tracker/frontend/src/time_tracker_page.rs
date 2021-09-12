use zoon::{*, eprintln};
use crate::{connection::connection, app};
use shared::{UpMsg, ClientId, ProjectId, TimeEntryId, InvoiceId, time_tracker};
use std::sync::Arc;

mod view;

const TIME_ENTRY_BREAKPOINT: u32 = 630;

// ------ ------
//     Types
// ------ ------

#[derive(Default)]
struct Client {
    id: ClientId,
    name:String,
    projects: Vec<Arc<Project>>,
}

#[derive(Default)]
struct Project {
    id: ProjectId,
    name: String,
    time_entries: MutableVec<Arc<TimeEntry>>,
}

#[derive(Default)]
struct TimeEntry {
    id: TimeEntryId,
    name: Mutable<String>,
    started: Mutable<Wrapper<DateTime<Local>>>,
    stopped: Mutable<Option<Wrapper<DateTime<Local>>>>,
    is_old: bool,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn clients() -> &'static MutableVec<Arc<Client>> {
    MutableVec::new()
}

#[static_ref]
fn current_time() -> &'static Mutable<DateTime<Local>> {
    current_time_updater();
    Mutable::new(Local::now())
}

#[static_ref]
fn current_time_updater() -> &'static Mutable<Timer> {
    Mutable::new(Timer::new(1_000, || current_time().set_neq(Local::now())))
}

// ------ ------
//   Commands
// ------ ------

pub fn request_clients() {
    Task::start(async {
        let msg = UpMsg::GetTimeTrackerClients;
        if let Err(error) = connection().send_up_msg(msg).await {
            eprintln!("get TimeTracker clients request failed: {}", error);
        }
    });
}

pub fn convert_and_set_clients(new_clients: Vec<time_tracker::Client>) {
    fn convert_clients(clients: Vec<time_tracker::Client>) -> Vec<Arc<Client>> {
        clients.into_iter().map(|client| {
            Arc::new(Client {
                id: client.id,
                name: client.name,
                projects: convert_projects(client.projects),
            })
        }).collect()
    }
    fn convert_projects(time_blocks: Vec<time_tracker::Project>) -> Vec<Arc<Project>> {
        time_blocks.into_iter().map(|project| {
            Arc::new(Project {
                id: project.id,
                name: project.name,
                time_entries: MutableVec::new_with_values(convert_time_entries(project.time_entries)),
            })
        }).collect()
    }
    fn convert_time_entries(time_entries: Vec<time_tracker::TimeEntry>) -> Vec<Arc<TimeEntry>> {
        time_entries.into_iter().map(|time_entry| {
            Arc::new(TimeEntry {
                id: time_entry.id,
                name: Mutable::new(time_entry.name),
                started: Mutable::new(Wrapper::new(time_entry.started)),
                stopped: Mutable::new(time_entry.stopped.map(Wrapper::new)),
                is_old: true,
            })
        }).collect()
    }
    clients().lock_mut().replace_cloned(convert_clients(new_clients));
}

// -- project --

fn toggle_tracker(project: &Project) {
    let active_time_entry = project
        .time_entries
        .lock_ref()
        .iter()
        .find(|time_entry| time_entry.stopped.get().is_none())
        .cloned(); 

    if let Some(active_time_entry) = active_time_entry {
        return active_time_entry.stopped.set(Some(Local::now().into()));
    } 
    add_time_entry(project);
}

// -- time_entry --

fn add_time_entry(project: &Project) {
    let mut time_entries = project.time_entries.lock_mut();

    let name = time_entries
        .first()
        .map(|time_entry| time_entry.name.get_cloned())
        .unwrap_or_default();

    let time_entry = TimeEntry::default();
    time_entry.name.set(name);
    // @TODO send up_msg
    time_entries.insert_cloned(0, Arc::new(time_entry));
}

fn delete_time_entry(project: &Project, time_entry_id: TimeEntryId) {
    // @TODO send up_msg + confirm dialog
    project.time_entries.lock_mut().retain(|time_entry| time_entry.id != time_entry_id);
}

fn rename_time_entry(time_entry_id: TimeEntryId, name: &str) {
    // @TODO send up_msg
    zoon::println!("rename_time_entry not implemented yet");
}

fn set_time_entry_started(time_entry: &TimeEntry, started: DateTime<Local>) {
    // @TODO send up_msg
    time_entry.started.set(Wrapper::new(started));
}

fn set_time_entry_stopped(time_entry: &TimeEntry, stopped: DateTime<Local>) {
    // @TODO send up_msg
    time_entry.stopped.set(Some(Wrapper::new(stopped)));
}

// ------ ------
//    Signals
// ------ ------

fn show_wide_time_entry() -> impl Signal<Item = bool> {
    app::viewport_width().signal().map(|width| width > TIME_ENTRY_BREAKPOINT).dedupe()
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
