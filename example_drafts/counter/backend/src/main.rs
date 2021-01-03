use moon::*;

async fn init() {}

async fn request_handler(req: Request) {}

fn main() {
    start!(init, request_handler);
}
