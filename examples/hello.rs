use async_std::net::TcpListener;
use mitey::Mitey;
use std::fmt;

#[async_std::main]
async fn main() -> http_types::Result<()> {
    // A bit inconvenient, but create the tcp connection manually.
    // This allows the decoupling from the runtime
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;

    Mitey::new()
        .with_state(HelloState("mitey mitey".to_owned()))
        .build()
        .serve(listener)
        .await
}

#[derive(Debug, Clone)]
struct HelloState(String);

impl mitey::State for HelloState {}

impl fmt::Display for HelloState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

