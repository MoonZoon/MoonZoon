use moon::*;

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new().title("Counters with macros example")
}

async fn up_msg_handler(_: UpMsgRequest) {}

fn main() {
    start!(init, frontend, up_msg_handler).unwrap();
}
