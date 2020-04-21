use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use async_std::task;
use http_types::{Request, Response, StatusCode};
use mitey::{Router, State};

#[async_std::main]
async fn main() -> http_types::Result<()> {
    // server 1
    let state_1 = State::init("mitey-state_1".to_owned());

    let mut router_1 = Router::new();
    router_1.add_route("/one", handle_one);
    let router_1 = router_1.init();

    let listener_1 = TcpListener::bind(("127.0.0.1", 8080)).await?;
    let addr_1 = format!("http://{}", listener_1.local_addr()?); // TODO fix hack
    println!("mitey serving at {}", addr_1);

    let incoming_1 = listener_1.incoming();

    let serve_1 = incoming_1.for_each(|stream| {
        let stream = stream.unwrap();

        let addr_1 = addr_1.clone();
        let state_1 = state_1.clone();
        let router_1 = router_1.clone();

        // Now spawn into mitey handler, with all components
        task::spawn( async move {
            if let Err(err) = mitey::accept(addr_1, stream, state_1, router_1).await {
                eprintln!("{}", err);
            }
        });
    });

    // server 2
    let state_2 = State::init("mitey-state_2".to_owned());

    let mut router_2 = Router::new();
    router_2.add_route("/two", handle_two);
    let router_2 = router_2.init();

    let listener_2 = TcpListener::bind(("127.0.0.1", 8081)).await?;
    let addr_2 = format!("http://{}", listener_2.local_addr()?); // TODO fix hack
    println!("mitey serving at {}", addr_2);

    let incoming_2 = listener_2.incoming();

    let serve_2 = incoming_2.for_each(|stream| {
        let stream = stream.unwrap();

        let addr_2 = addr_2.clone();
        let state_2 = state_2.clone();
        let router_2 = router_2.clone();

        // Now spawn into mitey handler, with all components
        task::spawn( async move {
            if let Err(err) = mitey::accept(addr_2, stream, state_2, router_2).await {
                eprintln!("{}", err);
            }
        });
    });

    // Combine streams (should use just one async runtime)
    futures_util::future::join(serve_1, serve_2).await;

    Ok(())
}

async fn handle_one(_req: Request) -> http_types::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain")?;
    res.set_body("mitey: small and mighty 1");

    //task::spawn_blocking(|| {
    task::block_on(async {
        println!("long-running task_1 started");
        std::thread::sleep(std::time::Duration::from_millis(10_000));
        println!("long-running task_1 ended");
    });
    Ok(res)
}

async fn handle_two(_req: Request) -> http_types::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/plain")?;
    res.set_body("mitey: small and mighty 2");

    //task::spawn_blocking(|| {
    task::block_on(async {
        println!("long-running task_2 started");
        std::thread::sleep(std::time::Duration::from_millis(10_000));
        println!("long-running task_2 ended");
    });
    Ok(res)
}
