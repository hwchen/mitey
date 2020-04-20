use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use async_std::task;
use http_types::{Request, Response, StatusCode};
use mitey::{Router, State};
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_std::main]
async fn main() -> http_types::Result<()> {
    // State (database)
    let state = Arc::new(Mutex::new(State("mitey mitey".to_owned())));

    // Routing
    let mut router = Router::new();
    router.add_route("/one", |_req: Request| async {
        println!("hit");
        let mut res = Response::new(StatusCode::Ok);
        res.insert_header("Content-Type", "text/plain")?;
        res.set_body("mitey: small and mighty");
        Ok(res)
    });
    //router.add_route("two", |_req| async {
    //    let mut res = Response::new(StatusCode::Ok);
    //    res.insert_header("Content-Type", "text/plain")?;
    //    res.set_body("mitey: minimal code, maximal power");
    //    Ok(res)
    //});
    let router = Arc::new(Mutex::new(router));

    // Now tcp IO

    // A bit inconvenient, but create the tcp connection manually.
    // This allows the decoupling from the runtime
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;

    let addr = listener.local_addr()?;
    println!("mitey serving at {}", addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        // TODO fix hack
        let addr = format!("http://{}", addr);

        let state = state.clone();
        let router = router.clone();

        // Now spawn into mitey handler, with all components
        task::spawn( async {
            if let Err(err) = mitey::accept(addr, stream, state, router).await {
                eprintln!("{}", err);
            }
        });
    }

    Ok(())
}
