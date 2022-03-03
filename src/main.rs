use hyper::service::{make_service_fn, service_fn};
use hyper::{self, Body, Request, Response, Server};
use std::net::SocketAddr;

async fn tunkki(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await?;

    println!("{}", std::str::from_utf8(&whole_body).unwrap());

    Ok(Response::new(Body::empty()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9099));

    let make_service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(tunkki)) });

    Server::bind(&addr).serve(make_service).await?;

    Ok(())
}
