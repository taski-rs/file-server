use std::{error::Error, io, net::SocketAddr, path::Path};
use tiny_http::{Header, Response, Server, StatusCode};

#[inline(never)]
pub fn serve(path: &Path, addr: Option<SocketAddr>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = addr.unwrap_or_else(|| ([0, 0, 0, 0], 8000).into());
    eprintln!("Serving {} on {}", path.display(), addr);

    let server = Server::http(addr)?;
    for request in server.incoming_requests() {
        eprintln!("request={:?}", request);
        if request.url().ends_with("/") {
            let response = Response::from_string("not implemented");
            request.respond(response)?;
        } else {
            let path = path.join(&request.url().get(1..).unwrap_or(""));
            if path.is_dir() {
                let redirect = format!("{}/", request.url());
                request.respond(Response::new(
                    StatusCode::from(301),
                    vec![Header::from_bytes("Location", redirect).unwrap()],
                    io::Cursor::new(vec![]),
                    None,
                    None,
                ))?;
                continue;
            }
            match std::fs::File::open(path) {
                Ok(file) => request.respond(Response::from_file(file))?,
                Err(err) => {
                    let status = match err.kind() {
                        io::ErrorKind::PermissionDenied => StatusCode::from(403),
                        io::ErrorKind::NotFound => StatusCode::from(404),
                        _ => StatusCode::from(500),
                    };
                    request.respond(Response::new(
                        status,
                        vec![],
                        io::Cursor::new(vec![]),
                        None,
                        None,
                    ))?;
                }
            }
        }
    }

    Ok(())
}
