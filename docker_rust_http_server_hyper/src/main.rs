use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::runtime::Runtime;

/// A function handling a request.
async fn response_for_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

pub fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a `tokio` runtime.
    let runtime = Runtime::new()?;

    // Create a service factory.
    let service_factory =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(response_for_request)) });

    // Create a socket address.
    let socket_address: SocketAddr = ([0, 0, 0, 0], 8000).into();

    // Spawn the root task
    runtime.block_on(async {
        println!("Listening on http://{}", socket_address);

        // Create a server.
        let server = Server::bind(&socket_address).serve(service_factory);

        // Run the server.
        // TODO: map the error.
        if let Err(e) = server.await {
            eprintln!("server error: {e}");
        }
    });

    Ok(())
}
