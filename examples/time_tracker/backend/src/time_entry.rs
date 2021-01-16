use shared::{TimeEntryId, ProjectId};
use chrono::prelude::*;

actor!{
    #[args]
    struct TimeEntryArgs {
        project: ProjectId,
        time_entry: shared::time_tracker::TimeEntry,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<TimeEntryId, TimeEntryActor> {
        index("time_entry_by_id", |_| id())
    }

    #[index]
    fn by_project() -> Index<ProjectId, TimeEntryActor> {
        index("time_entry_by_project", |_| client())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<TimeEntryId> {
        p_var("id", |_| args().map(|args| args.time_entry.id))
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| args().map(|args| args.time_entry.name.clone()))
    }

    #[p_var]
    fn started() -> PVar<DateTime<Local>> {
        p_var("started", |_| args().map(|args| args.time_entry.id))
    }

    #[p_var]
    fn stopped() -> PVar<Option<DateTime<Local>>> {
        p_var("stopped", |_| args().map(|args| args.time_entry.stopped))
    }

    #[p_var]
    fn project() -> PVar<ProjectId> {
        p_var("project", |_| args().map(|args| args.time_entry.project))
    }

    // ------ Actor ------

    #[actor]
    struct TimeEntryActor;
    impl TimeEntryActor {
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
