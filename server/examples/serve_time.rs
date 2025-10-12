use docsmith_pal_real::PalReal;
use docsmith_server::server::DocsmithServer;

fn main() {
    let pal = PalReal::new();
    let result = DocsmithServer::new(pal).run();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    std::thread::park()
}
