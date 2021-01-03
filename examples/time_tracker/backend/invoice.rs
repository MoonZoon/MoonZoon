use shared::{InvoiceId, TimeBlockId};
use crate::project::{self, ProjectActor};

actor!{
    #[args]
    struct InvoiceArgs {
        time_block: TimeBlockId,
        id: InvoiceId,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<InvoiceId, InvoiceActor> {
        index("invoice_by_id", |_| id())
    }

    #[index]
    fn by_time_block() -> Index<ClientId, InvoiceActor> {
        index("invoice_by_time_block", |_| time_block())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<InvoiceId> {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn custom_id() -> PVar<String> {
        p_var("custom_id", |_| String::new())
    }

    #[p_var]
    fn url() -> PVar<String> {
        p_var("url", |_| String::new())
    }

    #[p_var]
    fn time_block() -> PVar<ClientId> {
        p_var("time_block", |_| args().time_block)
    }

    // ------ Actor ------

    #[actor]
    struct InvoiceActor;
    impl InvoiceActor {
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
