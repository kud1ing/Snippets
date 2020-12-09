use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

/// A function handling a request.
async fn repsponse_for_request(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a service factory.
    let service_factory =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(repsponse_for_request)) });

    // Create a socket address.
    let socket_address: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Create a server.
    let server = Server::bind(&socket_address).serve(service_factory);

    println!("Listening on http://{}", socket_address);

    server.await?;

    Ok(())
}
