use shared::{ClientId, ProjectId};

actor!{
    #[args]
    struct Project {
        client: ClientId,
        id: ProjectId,
    }  

    #[index]
    fn by_id() -> ProjectId {
        index("project_by_id", |_| None)
    }

    #[index]
    fn by_client() -> ClientId {
        index("project_by_client", |_| None)
    }

    #[p_var]
    fn id() -> ProjectId {
        p_var("id", &[by_id()], |_| args().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", &[], |_| String::new())
    }

    #[p_var]
    fn client() -> ClientId {
        p_var("client", &[by_client()], |_| args().client)
    }

    #[in_msg]
    enum InMsg {
        Remove,
        Rename(String),
    }

    #[in_msg_handler]
    fn in_msg_handler(in_msg: InMsg) {
        match msg {
            InMsg::Remove => {
                remove_actor();
            }
            InMsg::Rename(name) => {
                name().set(name);
            }
        }
    }
}
