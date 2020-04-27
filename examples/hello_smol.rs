use anyhow::Result;
use http_types::{Request, Response, StatusCode};
use mitey::{Router, State};
use std::net::TcpListener;

use piper::Arc;
use smol::{Async, Task};

fn main() -> Result<()> {
    smol::run(async {
        let state = State::init("mitey-state1".to_owned());

        let mut router = Router::new();
        router.add_route("/one", handle_one);
        let router = router.init();

        let listener = Async::<TcpListener>::bind("127.0.0.1:8080")?;
        let addr = format!("http://{}", listener.get_ref().local_addr()?); // TODO fix hack
        println!("mitey serving at {}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            println!("connection accepted");

            let stream = Arc::new(stream);

            let addr = addr.clone();
            let state = state.clone();
            let router = router.clone();

            // Now spawn into mitey handler, with all components
            let task = Task::spawn( async move {
                if let Err(err) = mitey::accept(addr, stream, state, router).await {
                    eprintln!("{}", err);
                }
            });

            task.detach();
        };
    })
}

async fn handle_one(_req: Request) -> http_types::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain")?;
    res.set_body("mitey: small and mighty");

    //task::spawn_blocking(|| {
    let task = Task::blocking(async {
        println!("long-running task1 started");
        smol::Timer::after(std::time::Duration::from_millis(10_000)).await;
        println!("long-running task1 ended");
    });

    task.detach();

    Ok(res)
}

