use shared::{ClientId, ProjectId, TimeBlockId};
use crate::project::{self, ProjectActor};
use crate::time_block::{self, TimeBlockActor};
use chrono::{prelude::*, Duration};

actor!{
    #[args]
    struct ClientArgs {
        id: ClientId,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<ClientId, ClientActor> {
        index("client_by_id", |_| id())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<ClientId> {
        p_var("id", |_| args().map(|args| args.id))
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| String::new())
    }

    // ------ Actor ------

    #[actor]
    struct ClientActor;
    impl ClientActor {
        async fn remove(&self) {
            let remove_project_futs = project::by_client()
                .actors(id().inner().await)
                .iter()
                .map(ProjectActor::remove);

            let remove_time_block_futs = time_block::by_client()
                .actors(id().inner().await)
                .iter()
                .map(TimeBlockActor::remove);

            join_all(remove_project_futs.chain(remove_time_block_futs)).await;
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn projects(&self) -> Vec<ProjectActor> {
            project::by_client().actors(id().inner().await)
        }

        async fn time_blocks(&self) -> Vec<TimeBlockActor> {
            time_blocks::by_client().actors(id().inner().await)
        }

        async fn id(&self) -> ClientId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }

        async fn tracked(&self) -> Duration {
            let now = chrono::Local::now();

            let time_entry_duration_fut = |time_entry| async {
                let (started, stopped) = join(
                    time_entry.started(), time_entry.stopped()
                ).await;
                stopped.unwrap_or(now) - started
            };

            let project_duration_fut = |project| async {
                let duration_futs = project
                        .time_entries()
                        .await
                        .iter()
                        .map(time_entry_duration_fut);
                join_all(durations_futs)
                    .await
                    .iter()
                    .fold(Duration::seconds(0), Duration::add)
            };

            let duration_futs = project::by_client()
                .actors(id().inner().await)
                .iter()
                .map(project_duration_fut);
            join_all(duration_futs)
                .await
                .iter()
                .fold(Duration::seconds(0), Duration::add)
        }
    }
}
