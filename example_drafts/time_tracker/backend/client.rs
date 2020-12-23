use shared::ClientId;
use crate::project::{self, ProjectActor};

actor!{
    #[args]
    struct Client {
        id: ClientId,
    }  

    #[index]
    fn by_id() -> ClientId {
        index("client_by_id", |_| None)
    }

    #[p_var]
    fn id() -> ClientId {
        p_var("id", &[by_id()], |_| args().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", &[], |_| String::new())
    }

    #[actor]
    struct ClientActor {
        async fn remove(&self) {
            let futures = project::by_id()
                .get(id().inner())
                .iter()
                .map(ProjectActor::remove)
                .collect();
    
            join_all(futures).await;
            self.remove_actor().await
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }
    }
}
