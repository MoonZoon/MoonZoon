use shared::{ClientId, ProjectId};

actor!{
    #[args]
    struct Project {
        client: ClientId,
        id: ProjectId,
    }  

    #[index]
    fn by_id() -> ProjectId {
        index("project_by_id", |_| id())
    }

    #[index]
    fn by_client() -> ClientId {
        index("project_by_client", |_| client())
    }

    #[p_var]
    fn id() -> ProjectId {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", |_| String::new())
    }

    #[p_var]
    fn client() -> ClientId {
        p_var("client", |_| args().client)
    }

    #[actor]
    struct ProjectActor {
        async fn remove(&self) {
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn id(&self) -> ProjectId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }
    }
}
