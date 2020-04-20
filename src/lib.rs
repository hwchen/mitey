// I think dependence on http_types is fine for now.
//
// Using a patched version of async_h1 that uses futures only, no async_std.

use futures_io::{AsyncRead, AsyncWrite};
use http_types::{Request, Response, StatusCode};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

/// compatibility with tokio streams
pub mod compat;

/// State placeholder
#[derive(Debug, Clone)]
pub struct State(String);

impl State {
    pub fn init(str: String) -> Arc<Mutex<State>> {
        Arc::new(Mutex::new(State(str)))
    }
}

/// placeholder
/// to become mitey-router
/// Could use a RouteBuilder
#[derive(Debug, Clone)]
pub struct Router<F, Fut>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{
    routes: Vec<(String, Box<F>)>,
}

impl<F, Fut> Router<F, Fut>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(&mut self, path: &str, endpoint: F) {
        self.routes.push((path.to_string(), Box::new(endpoint)));
    }

    pub fn init(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

}

pub async fn accept<RW, F, Fut>(
    addr: String,
    stream: RW,
    state: Arc<Mutex<State>>,
    router: Arc<Mutex<Router<F, Fut>>>,
) -> http_types::Result<()>
where
    RW: AsyncRead + AsyncWrite + Clone + Send + Sync + Unpin + 'static,
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{
    println!("connection received: {}", addr);
    println!("state: {:?}", state.lock().await);

    async_h1::accept(&addr, stream.clone(), |req: Request| async {
            let path = req.url().path();
            println!("{:?}", path);

            let router = router.lock().await;

            for route in &router.routes {
                if route.0 == path {
                    return route.1(req).await;
                }
            };

            Ok(Response::new(StatusCode::Ok))
    }).await?;

    Ok(())
}
