use shared::{AccessToken, UserId};
use std::collections::BTreeSet;

actor!{
    #[args]
    struct UserArgs;

    #[index]
    fn by_id() -> Index<UserId, UserActor> {
        index("user_by_id", |_| id())
    }

    #[p_var]
    fn id() -> PVar<UserId> {
        p_var("id", |_| UserId::new())
    }

    #[p_var]
    fn name() -> PVar<String> {
        p_var("name", |_| "John".to_owned())
    }

    #[p_var]
    fn password() -> PVar<String> {
        p_var("password", |_| "Password1".to_owned())
    }

    #[p_var]
    fn access_tokens() -> PVar<BTreeSet<Token>> {
        p_var("token", |_| BTreeSet::new())
    }

    #[actor]
    struct UserActor { 
        async fn login(&self, password: String) -> Option<shared::User> {
            if password().inner().await == password {
                let access_token = AccessToken::new();

                let (id, name, _) = join!(
                    id().inner(), 
                    name().inner(),
                    access_tokens().update_mut(|tokens| tokens.insert(access_token)),
                ).await;

                Some(shared::User { id, name, access_token })
            } else {
                None
            }
        }

        async fn logout(&self, access_token: AccessToken) {
            access_tokens().update_mut(|tokens| tokens.remove(access_token)).await;
        }

        async fn logged_in(&self, access_token: AccessToken) -> bool {
            access_tokens().map(|tokens| tokens.contains(access_token)).await
        }
    }
}
