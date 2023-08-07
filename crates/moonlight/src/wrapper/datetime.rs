use crate::*;

impl<Tz: TimeZone> From<Wrapper<Self>> for DateTime<Tz> {
    fn from(wrapper: Wrapper<Self>) -> Self {
        wrapper.inner
    }
}

impl Default for Wrapper<DateTime<Local>> {
    fn default() -> Self {
        Wrapper::new(Local::now())
    }
}
