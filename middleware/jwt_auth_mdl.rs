use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use log::info;

use crate::token_util;
use crate::service_error;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtAuth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for JwtAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMdl<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMdl { service })
    }
}

pub struct JwtAuthMdl<S> {
    service: S,
}

impl<S, B> Service for JwtAuthMdl<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut is_valid = true;
        if req.path() != "/login" {
			let headers = req.headers();
			let token_opt = headers.get("token");
            match token_opt {
                Some(token) => {
                    is_valid = token_util::validate_token(token.to_str().unwrap());
                },
                None => {
					info!("Token not found in header");
                    is_valid = false
                } 
            }
		}
        if is_valid {
            Box::new(self.service.call(req).and_then(|res| {
                Ok(res)
            }))
        } else {
			Box::new(self.service.call(req).and_then(|_res| {
				Err(service_error::ServiceError::Unauthorized.into())
			}))
        }
	}
}