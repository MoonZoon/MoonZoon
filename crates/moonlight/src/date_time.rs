use crate::*;

#[derive(Serialize, Deserialize)]
pub struct DateTime<Tz: TimeZone> {
    #[serde(skip)]
    tz: std::marker::PhantomData<Tz>
}
