// I think dependence on http_types is fine for now.
//
// Using a patched version of async_h1 that uses futures only, no async_std.

use futures_io::{AsyncRead, AsyncWrite};
use http_types::{Response, StatusCode};
use std::sync::{Arc, Mutex};

pub struct Mitey {
    state: Arc<Mutex<State>>,
    #[allow(dead_code)]
    router: Router,
}

impl Mitey {
    pub fn new() -> MiteyBuilder {
        MiteyBuilder::new()
    }

    pub fn state(&self) -> Arc<Mutex<State>> {
        self.state.clone()
    }
}

pub struct MiteyBuilder {
    state: Option<State>,
    router: Router,
}

impl MiteyBuilder {
    fn new() -> Self {
        Self {
            state: None,
            router: Router::new(),
        }
    }

    pub fn with_state(&mut self, state: State) -> &mut Self {
        self.state = Some(state);
        self
    }

    /// takes path and async fn -> Response
    pub fn add_route(&mut self, path: &str) -> &mut Self {
        self.router.add_route(path);
        self
    }

    /// finalize and build
    pub fn build(&self) -> Mitey {
        Mitey {
            state: self.state.clone().map(|s|Arc::new(Mutex::new(s))).expect("For now state is required"),
            router: self.router.clone(),
        }
    }
}

/// State placeholder
#[derive(Debug, Clone)]
pub struct State(pub String);

/// placeholder
/// to become mitey-router
#[derive(Debug, Clone)]
struct Router {
    paths: Vec<String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            paths: vec![],
        }
    }

    pub fn add_route(&mut self, path: &str) {
        self.paths.push(path.to_string());
    }
}

pub async fn accept<RW>(addr: String, stream: RW, state: Arc<Mutex<State>>) -> http_types::Result<()>
    where RW: AsyncRead + AsyncWrite + Clone + Send + Sync + Unpin + 'static
{
    println!("connection received: {}", addr);
    println!("state: {:?}", state.lock().unwrap());

    async_h1::accept(&addr, stream.clone(), |_req| async move {
        let mut res = Response::new(StatusCode::Ok);
        res.insert_header("Content-Type", "text/plain")?;
        res.set_body("mitey: small and mighty | ");
        Ok(res)
    })
    .await?;
    Ok(())
}

