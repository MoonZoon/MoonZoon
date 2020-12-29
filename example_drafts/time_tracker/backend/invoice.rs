use shared::{InvoiceId, TimeBlockId};
use crate::project::{self, ProjectActor};

actor!{
    #[args]
    struct Invoice {
        time_block: TimeBlockId,
        id: InvoiceId,
    }  

    #[index]
    fn by_id() -> InvoiceId {
        index("invoice_by_id", |_| id())
    }

    #[index]
    fn by_time_block() -> ClientId {
        index("invoice_by_time_block", |_| time_block())
    }

    #[p_var]
    fn id() -> InvoiceId {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn custom_id() -> String {
        p_var("custom_id", |_| String::new())
    }

    #[p_var]
    fn url() -> String {
        p_var("url", |_| String::new())
    }

    #[p_var]
    fn time_block() -> ClientId {
        p_var("time_block", |_| args().time_block)
    }

    #[actor]
    struct InvoiceActor {
        async fn remove(&self) {
            self.remove_actor().await
        }
    
        async fn set_custom_id(&self, custom_id: String) {
            custom_id().set(custom_id).await
        }

        async fn set_url(&self, url: String) {
            url().set(url).await
        }

        async fn id(&self) -> InvoiceId {
            id().inner().await
        }

        async fn custom_id(&self) -> String {
            custom_id().inner().await
        }

        async fn url(&self) -> String {
            url().inner().await
        }
    }
}
