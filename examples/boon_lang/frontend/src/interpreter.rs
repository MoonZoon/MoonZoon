use zoon::{*, println};
use zoon::futures_util::stream;
use std::sync::Arc;


pub async fn run(_program: &str) -> impl Element {
    El::new().child("3. attempt")
}
