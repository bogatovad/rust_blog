// use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
// use actix_web::http::header;
// use futures_util::future::{Ready, ok};
// use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
// use std::{future::Future, pin::Pin, task::{Context, Poll}};
//
// #[derive(Clone)]
// pub struct JwtAuth { secret: String }
//
// impl JwtAuth {
//     pub fn new(secret: impl Into<String>) -> Self { Self { secret: secret.into() } }
// }
//
// impl<S, B> Transform<S, ServiceRequest> for JwtAuth
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = JwtAuthSvc<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(JwtAuthSvc { service, secret: self.secret.clone() })
//     }
// }
//
// pub struct JwtAuthSvc<S> { service: S, secret: String }
//
// impl<S, B> Service<ServiceRequest> for JwtAuthSvc<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//
//     fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }
//
//     fn call(&self, mut req: ServiceRequest) -> Self::Future {
//         let token = req.headers()
//             .get(header::AUTHORIZATION)
//             .and_then(|h| h.to_str().ok())
//             .and_then(|s| s.strip_prefix("Bearer "))
//             .map(str::to_owned);
//
//         let Some(token) = token else {
//             let (r, _) = req.into_parts();
//             return Box::pin(async move {
//                 Ok(ServiceResponse::new(r, HttpResponse::Unauthorized().json(serde_json::json!({"error": "missing bearer"}))))
//             });
//         };
//
//         let secret = self.secret.clone();
//         let fut_service = self.service.call;
//
//         Box::pin(async move {
//             let mut validation = Validation::new(Algorithm::HS256);
//             validation.validate_exp = true;
//             validation.leeway = 60;
//
//             let token_data = match decode::<serde_json::Value>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
//                 Ok(td) => td,
//                 Err(_) => {
//                     let (r, _) = req.into_parts();
//                     return Ok(ServiceResponse::new(r, HttpResponse::Unauthorized().json(serde_json::json!({"error": "invalid token"}))));
//                 }
//             };
//
//             req.extensions_mut().insert(token_data.claims);
//             fut_service(req).await
//         })
//     }
// }