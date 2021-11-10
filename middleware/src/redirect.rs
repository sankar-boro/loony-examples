use std::task::{Context, Poll};

use futures::future::{ok, Either, Ready};
use loony::web::{WebRequest, WebResponse};
use loony::web::{Error, HttpResponse};
use loony::{http, Service, Transform};

pub struct CheckLogin;

impl<S, Err> Transform<S> for CheckLogin
where
    S: Service<Request = WebRequest<Err>, Response = WebResponse, Error = Error>,
    S::Future: 'static,
{
    // type Request = WebRequest<Err>;
    // type Response = WebResponse;
    // type Error = Error;
    // type InitError = ();
    // type Transform = CheckLoginMiddleware<S>;
    // type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Service = CheckLoginMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Service {
        CheckLoginMiddleware { service }
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, Err> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = WebRequest<Err>, Response = WebResponse, Error = Error>,
{
    type Request = WebRequest<Err>;
    type Response = WebResponse;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: Self::Request) -> Self::Future {
        // We only need to hook into the `start` for this middleware.

        let is_logged_in = false; // Change this to see the change in outcome in the browser

        if is_logged_in {
            Either::Left(self.service.call(req))
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() == "/login" {
                Either::Left(self.service.call(req))
            } else {
                Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/login")
                        .finish()
                        .into_body(),
                )))
            }
        }
    }
}
