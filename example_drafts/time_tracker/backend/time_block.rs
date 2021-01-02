use shared::{ClientId, TimeBlockId, time_blocks::TimeBlockStatus};
use crate::invoice::{self, InvoiceActor};
use chrono::{prelude::*, Duration};

actor!{
    #[args]
    struct TimeBlockArgs {
        client: ClientId,
        id: TimeBlockId,
        duration: Duration,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<TimeBlockId, TimeBlockActor> {
        index("time_block_by_id", |_| id())
    }

    #[index]
    fn by_client() -> Index<ClientId, TimeBlockActor> {
        index("time_block_by_client", |_| client())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<TimeBlockId> {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| String::new())
    }

    #[p_var]
    fn status() -> PVar<TimeBlockStatus> {
        p_var("status", |_| TimeBlockStatus::Unpaid)
    }

    #[p_var]
    fn duration() -> PVar<Duration> {
        p_var("duration", |_| args().duration)
    }

    #[p_var]
    fn client() -> PVar<ClientId> {
        p_var("client", |_| args().client)
    }

    // ------ Actor ------

    #[actor]
    struct TimeBlocktActor;
    impl TimeBlockActor {
        async fn remove(&self) {
            let invoice = invoice::by_time_block().actors(id().inner().await).first();
            if let Some((_, invoice)) = invoice {
                invoice.remove().await;
            }
            self.remove_actor().await;
        }
    
        async fn rename(&self, name: String) {
            name().set(name).await
        }

        async fn set_status(&self, status: TimeBlockStatus) {
            status().set(status).await
        }

        async fn set_duration(&self, duration: Duration) {
            duration().set(duration).await
        }

        async fn id(&self) -> TimeBlockId {
            id().inner().await
        }

        async fn name(&self) -> String {
            name().inner().await
        }

        async fn status(&self) -> TimeBlockStatus {
            status().inner().await
        }

        async fn duration(&self) -> Duration {
            duration().inner().await
        }

        async fn invoice(&self) -> Option<InvoiceActor> {
            invoice::by_time_block()
                .actors(id().inner().await)
                .first()
        }
    }
}
