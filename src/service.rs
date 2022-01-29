use std::convert::Infallible;
use std::path::PathBuf;

use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Body, Method, Request, Response, StatusCode};

use crate::spelunkicon::Spelunkicon;

const MAX_INPUT: usize = 64;
static PNG_SUFFIX: &str = ".png";

pub async fn spelunkicon(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = PathBuf::from(req.uri().path());
    println!("Request: {:?}", path);
    let mut response = Response::new(Body::empty());

    if req.method() != Method::GET {
        *response.status_mut() = StatusCode::NOT_FOUND;
        return Ok(response);
    }

    // Verify that there's a filename in the path
    let filename = match path.file_name() {
        Some(filename) => String::from(filename.to_string_lossy()),
        None => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Ok(response);
        }
    };

    // Verify that a PNG has been requested
    if !filename.ends_with(PNG_SUFFIX) {
        *response.status_mut() = StatusCode::NOT_FOUND;
        return Ok(response);
    }

    // Strip the PNG suffix
    let input = String::from(&filename[..filename.len() - PNG_SUFFIX.len()]);

    // Validate the input length
    if input.len() <= 0 || input.len() >= MAX_INPUT {
        *response.status_mut() = StatusCode::NOT_FOUND;
        return Ok(response);
    }

    // Generate the PNG
    let config = Spelunkicon::from_input(&input);
    let png = match config.to_png() {
        Some(png) => png,
        None => {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            *response.body_mut() = Body::from("Something went wrong...");
            return Ok(response);
        }
    };

    // Serve the request
    let headers = response.headers_mut();
    let content_type = HeaderValue::from_static("image/png");
    headers.insert(CONTENT_TYPE, content_type);
    *response.body_mut() = Body::from(png);
    Ok(response)
}
