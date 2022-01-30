use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::service::Service;
use hyper::{Body, Method, Request, Response, StatusCode};

use crate::generator::Generator;
use crate::spelunkicon::Spelunkicon;

const MAX_INPUT: usize = 64;
static PNG_SUFFIX: &str = ".png";

pub struct IconService {
    pub generator: Arc<Generator>,
}

impl IconService {
    pub fn new(generator: Arc<Generator>) -> Self {
        Self { generator }
    }
}

impl Service<Request<Body>> for IconService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = PathBuf::from(req.uri().path());
        println!("Request: {:?}", path);
        let mut response = Response::new(Body::empty());

        if req.method() != Method::GET {
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Box::pin(async { Ok(response) });
        }

        // Verify that there's a filename in the path
        let filename = match path.file_name() {
            Some(filename) => String::from(filename.to_string_lossy()),
            None => {
                *response.status_mut() = StatusCode::NOT_FOUND;
                return Box::pin(async { Ok(response) });
            }
        };

        // Verify that a PNG has been requested
        if !filename.ends_with(PNG_SUFFIX) {
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Box::pin(async { Ok(response) });
        }

        // Strip the PNG suffix
        let input = String::from(&filename[..filename.len() - PNG_SUFFIX.len()]);

        // Validate the input length
        if input.len() <= 0 || input.len() >= MAX_INPUT {
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Box::pin(async { Ok(response) });
        }

        // Generate the PNG
        let config = Spelunkicon::from_input(&input);
        let png = match self.generator.make_png(config) {
            Some(png) => png,
            None => {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *response.body_mut() = Body::from("Something went wrong...");
                return Box::pin(async { Ok(response) });
            }
        };

        // Serve the request
        let headers = response.headers_mut();
        let content_type = HeaderValue::from_static("image/png");
        headers.insert(CONTENT_TYPE, content_type);
        *response.body_mut() = Body::from(png);

        Box::pin(async { Ok(response) })
    }
}

pub struct MakeIconService {
    pub generator: Arc<Generator>,
}

impl<T> Service<T> for MakeIconService {
    type Response = IconService;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let generator = self.generator.clone();
        let fut = async move { Ok(Self::Response { generator }) };

        Box::pin(fut)
    }
}
