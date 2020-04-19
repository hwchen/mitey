use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use async_std::task;
use mitey::{Mitey, State};

#[async_std::main]
async fn main() -> http_types::Result<()> {
    let mitey = Mitey::new()
        .with_state(State("mitey mitey".to_owned()))
        .build();

    // A bit inconvenient, but create the tcp connection manually.
    // This allows the decoupling from the runtime
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;

    let addr = listener.local_addr()?;
    println!("mitey serving at {}", addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        // is it weird I need to pass the addr down?
        // Also, it's an ugly hack to require adding the base, it's because of
        // how they decode
        let addr = format!("http://{}", addr);

        let state = mitey.state();
        task::spawn(async {
            if let Err(err) = mitey::accept(addr, stream, state).await {
                eprintln!("{}", err);
            }
        });

    }

    Ok(())
}


