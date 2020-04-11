// I think dependence on http_types is fine for now. And maybe even async_h1. Should try to move
// away from async_std runtime.
//
// async_std tcp, can I build my own tcp over AsyncRead and AsyncWrite? One that is executor
// agnostic? async_std may already be, I think it's build on std types.
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use http_types::{Response, StatusCode};

use std::sync::{Arc, Mutex};

pub struct Mitey<S: State> {
    state: Arc<Mutex<S>>,
    #[allow(dead_code)]
    router: Router,
}

impl<S: State> Mitey<S> {
    pub fn new() -> MiteyBuilder<S> {
        MiteyBuilder::new()
    }

    pub async fn serve(self, connection: TcpListener) -> http_types::Result<()> {
        let addr = connection.local_addr()?;
        println!("mitey serving at {}", addr);

         let mut incoming = connection.incoming();
         while let Some(stream) = incoming.next().await {
             let stream = stream?;

             // is it weird I need to pass the addr down?
             // Also, it's an ugly hack to require adding the base, it's because of
             // how they decode
             let addr = format!("http://{}", addr);

             let state = self.state.clone();
             task::spawn(async {
                 if let Err(err) = accept(addr, stream, state).await {
                     eprintln!("{}", err);
                 }
             });
         }

         Ok(())
    }
}

async fn accept(addr: String, stream: TcpStream, state: Arc<Mutex<impl State>>) -> http_types::Result<()> {
    println!("connection received: {}", stream.peer_addr()?);
    println!("state: {}", state.lock().unwrap().to_string());

    async_h1::accept(&addr, stream.clone(), |_req| async move {
        let mut res = Response::new(StatusCode::Ok);
        res.insert_header("Content-Type", "text/plain")?;
        res.set_body("mitey: small and mighty | ");
        Ok(res)
    })
    .await?;
    Ok(())
}


pub struct MiteyBuilder<S: State> {
    state: Option<S>,
    router: Router,
}

impl<S: State> MiteyBuilder<S> {
    fn new() -> Self {
        Self {
            state: None,
            router: Router::new(),
        }
    }

    pub fn with_state(&mut self, state: S) -> &mut Self {
        self.state = Some(state);

        self
    }

    /// takes path and async fn -> Response
    pub fn add_route(&mut self, path: &str) -> &mut Self {
        self.router.add_route(path);

        self
    }

    /// finalize, build, serve at address
    ///
    /// Currently takes an async_std TcpStream.
    /// TODO design to take TcpStream trait, which any executor can implement (and trivially, if it
    /// uses std AsyncReadWrite types.
    pub fn build(&self) -> Mitey<S> {
        Mitey {
            state: self.state.clone().map(|s|Arc::new(Mutex::new(s))).expect("For now state is required"),
            router: self.router.clone(),
        }
    }
}

// TODO remove ToString later.
pub trait State: ToString + Clone + Send + Sync + 'static {}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
