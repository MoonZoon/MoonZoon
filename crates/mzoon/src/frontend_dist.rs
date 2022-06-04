use anyhow::Error;
use fehler::throws;

// -- public --

#[throws]
pub async fn create_frontend_dist() {
    println!("Creating frontend_dist...");

    println!("frontend_dist created");
}
