use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use async_std::task;
use http_types::{Request, Response, StatusCode};
use mitey::{Router, State};

#[async_std::main]
async fn main() -> http_types::Result<()> {
    // State (database)
    let state = State::init("mitey-state".to_owned());

    // Routing
    // not a real router, so can't add another route
    let mut router = Router::new();
    router.add_route("/one", handle_one);
    let router = router.init();

    // Now tcp IO
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;
    let addr = listener.local_addr()?;
    let addr = format!("http://{}", addr); // TODO fix hack
    println!("mitey serving at {}", addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        let addr = addr.clone();
        let state = state.clone();
        let router = router.clone();

        // Now spawn into mitey handler, with all components
        task::spawn( async move {
            if let Err(err) = mitey::accept(addr, stream, state, router).await {
                eprintln!("{}", err);
            }
        });
    }

    Ok(())
}

async fn handle_one(_req: Request) -> http_types::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain")?;
    res.set_body("mitey: small and mighty");

    //task::spawn_blocking(|| {
    task::block_on(async {
        println!("long-running task started");
        std::thread::sleep(std::time::Duration::from_millis(10_000));
        println!("long-running task ended");
    });
    Ok(res)
}
