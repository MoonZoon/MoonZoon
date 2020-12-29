use shared::{ClientId, InvoiceId, TimeBlockId, time_blocks::TimeBlockStatus};
use crate::invoice::{self, InvoiceActor};
use chrono::{prelude::*, Duration};

actor!{
    #[args]
    struct TimeBlock {
        client: ClientId,
        id: TimeBlockId,
        duration: Duration,
    }  

    #[index]
    fn by_id() -> TimeBlockId {
        index("time_block_by_id", |_| id())
    }

    #[index]
    fn by_client() -> ClientId {
        index("time_block_by_client", |_| client())
    }

    #[p_var]
    fn id() -> TimeBlockId {
        p_var("id", |_| args().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", |_| String::new())
    }

    #[p_var]
    fn status() -> TimeBlockStatus {
        p_var("status", |_| TimeBlockStatus::Unpaid)
    }

    #[p_var]
    fn duration() -> Duration {
        p_var("duration", |_| args().duration)
    }

    #[p_var]
    fn client() -> ClientId {
        p_var("client", |_| args().client)
    }

    #[actor]
    struct TimeBlocktActor {
        async fn remove(&self) {
            let invoice = invoice::by_time_block().get(id().inner().await).first();
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

        async fn invoice(&self) -> Option<(InvoiceId, InvoiceActor)> {
            invoice::by_time_block()
                .get(id().inner().await)
                .first()
        }
    }
}
