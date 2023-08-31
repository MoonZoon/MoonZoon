use frontend::MarkdownWebWorker;
use gloo_worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();
    MarkdownWebWorker::registrar().register();
}
