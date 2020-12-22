use shared::ClientId;
use crate::project;

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

    #[in_msg]
    enum InMsg {
        Remove,
        Rename(String),
    }

    #[in_msg_handler]
    fn in_msg_handler(in_msg: InMsg) {
        match msg {
            InMsg::Remove => {
                for project in project::by_id(id().inner()) {
                    project.send_in_msg(project::InMsg::Remove)
                }
                remove_actor();
            }
            InMsg::Rename(name) => {
                name().set(name);
            }
        }
    }
}
