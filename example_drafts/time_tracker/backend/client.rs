use shared::{ClientId, ProjectId, TimeBlockId};
use crate::project::{self, ProjectActor};
use crate::time_block::{self, TimeBlockActor};

actor!{
    #[args]
    struct Client {
        id: ClientId,
    }  

    #[index]
    fn by_id() -> Index<ClientId, ClientActor> {
        index("client_by_id", |_| id())
    }

    #[p_var]
    fn id() -> PVar<ClientId> {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| String::new())
    }

    #[actor]
    struct ClientActor {
        async fn remove(&self) {
            let remove_project_futs = project::by_client()
                .get(id().inner().await)
                .iter()
                .map(|(_, project)| project.remove());

            let remove_time_block_futs = time_block::by_client()
                .get(id().inner().await)
                .iter()
                .map(|(_, time_block)| time_block.remove());

            join_all(remove_project_futs.chain(remove_time_block_futs)).await;
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn projects(&self) -> Vec<(ProjectId, ProjectActor)> {
            project::by_client()
                .get(id().inner().await)
                .iter()
                .collect()
        }

        async fn time_blocks(&self) -> Vec<(TimeBlockId, TimeBlockActor)> {
            time_blocks::by_client()
                .get(id().inner().await)
                .iter()
                .collect()
        }

        async fn id(&self) -> ClientId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }
    }
}
