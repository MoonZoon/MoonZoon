use shared::{TimeEntryId, ProjectId};
use chrono::prelude::*;

actor!{
    #[args]
    struct TimeEntry {
        project: ProjectId,
        time_entry: shared::time_tracker::TimeEntry,
    }  

    #[index]
    fn by_id() -> TimeEntryId {
        index("time_entry_by_id", |_| id())
    }

    #[index]
    fn by_project() -> ProjectId {
        index("time_entry_by_project", |_| client())
    }

    #[p_var]
    fn id() -> TimeEntryId {
        p_var("id", |_| args().time_entry.id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", |_| args().time_entry.name.clone())
    }

    #[p_var]
    fn started() -> DateTime<Local> {
        p_var("started", |_| args().time_entry.started)
    }

    #[p_var]
    fn stopped() -> Option<DateTime<Local>> {
        p_var("stopped", |_| args().time_entry.stopped)
    }

    #[p_var]
    fn project() -> ProjectId {
        p_var("project", |_| args().project)
    }

    #[actor]
    struct TimeEntryActor {
        async fn remove(&self) {
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn set_started(&self, started: DateTime<Local>) {
            started().set(started).await
        }

        async fn set_stopped(&self, stopped: DateTime<Local>) {
            stopped().set(stopped).await
        }

        async fn id(&self) -> TimeEntryId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }
    }
}
