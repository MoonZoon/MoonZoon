use shared::{ClientId, ProjectId};
use crate::time_entry::{self, TimeEntryActor};

actor!{
    #[args]
    struct ProjectArgs {
        client: ClientId,
        id: ProjectId,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<ProjectId, ProjectActor> {
        index("project_by_id", |_| id())
    }

    #[index]
    fn by_client() -> Index<ProjectId, ProjectActor> {
        index("project_by_client", |_| client())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<ProjectId> {
        p_var("id", |_| args().map(|args| args.id))
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| String::new())
    }

    #[p_var]
    fn client() -> PVar<ClientId> {
        p_var("client", |_| args().map(|args| args.client))
    }

    // ------ Actor ------

    #[actor]
    struct ProjectActor;
    impl ProjectActor {
        async fn remove(&self) {
            let remove_time_entry_futs = time_entry::by_project()
                .actors(id().inner().await)
                .iter()
                .map(TimeEntryActor::remove);

            join_all(remove_time_entry_futs).await;
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn time_entries(&self) -> Vec<TimeEntryActor> {
            time_entry::by_project().actors(id().inner().await)
        }

        async fn id(&self) -> ProjectId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }
    }
}
