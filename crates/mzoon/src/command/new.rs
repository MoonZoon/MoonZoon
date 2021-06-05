use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn new(_project_name: String, _here: bool) {
    println!("Command `new` has not been implemented yet")
}
