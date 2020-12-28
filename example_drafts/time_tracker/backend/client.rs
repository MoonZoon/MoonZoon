use shared::ClientId;
use crate::project::{self, ProjectActor};

actor!{
    #[args]
    struct Client {
        id: ClientId,
    }  

    #[index]
    fn by_id() -> ClientId {
        index("client_by_id", |_| id())
    }

    #[p_var]
    fn id() -> ClientId {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", |_| String::new())
    }

    #[actor]
    struct ClientActor {
        async fn remove(&self) {
            let remove_project_futs = project::by_client()
                .get(id().inner().await)
                .iter()
                .map(|(_, project)| project.remove());
            join_all(remove_project_futs).await;
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

        async fn id(&self) -> ClientId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }
    }
}
