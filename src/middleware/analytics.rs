use crate::extractors::analytics as AnalyticsType;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixWebError,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
pub struct Analytics;
pub struct AnalyticsMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for Analytics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = (); // indicates an error that might occur when creating the middleware instance
    type Transform = AnalyticsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // new_transform creates a new instance of the middleware Service.
    // The created middleware should wrap the service indicated by the service parameter
    //
    // returns a Future to allow some asynchronous work to be done while creating the middleware.
    fn new_transform(&self, service: S) -> Self::Future {
        // We only need to create a new object, so we'll use a Ready future to wrap the new middleware inside a future.
        // This is similar to using Javascript's Promise.resolve to place a value inside a Promise
        ready(Ok(AnalyticsMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AnalyticsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static, // B type paramter here represents the type of the body returned from the service
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    // Makes it easier to use an async block without needing to deal with the opaque future
    // types returned by async blocks.
    // LocalBoxFuture is the non-Send version of BoxFuture
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Actix calls poll_ready to determine if the service is ready to be invoked.
    // forward_ready! macro delegates this function to the wrapped service
    forward_ready!(service);

    // The call function is where all the "real" functionality goes.
    // You can inspect or mutate the request and response objects as needed,
    // and invoke the wrapped service if appropriate.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let x = AnalyticsType::Analytics::new(req.headers(),  req.peer_addr().unwrap().to_string());
        println!("{:?}",x);

        let fut = self.service.call(req);
        //HeaderMap { inner: {"accept": Value { inner: ["text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"] }, "host": Value { inner: ["127.0.0.1:8001"] }, "sec-fetch-user": Value { inner: ["?1"] }, "user-agent": Value { inner: ["Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/109.0"] }, "sec-fetch-site": Value { inner: ["none"] }, "sec-fetch-mode": Value { inner: ["navigate"] }, "connection": Value { inner: ["keep-alive"] }, "upgrade-insecure-requests": Value { inner: ["1"] }, "accept-language": Value { inner: ["en-US,en;q=0.5"] }, "sec-fetch-dest": Value { inner: ["document"] }, "accept-encoding": Value { inner: ["gzip, deflate, br"] }} }
        Box::pin(async move {
            let res = fut.await?;
            // let body = res.clone().into_body();
            // let (http_req, payload): (&HttpRequest, &Payload) = res.parts();
            // let auth_body: web::Json<AuthenticationBody> = web::Json::<AuthenticationBody>::from_request(http_req, &mut *payload).await.unwrap();
            // let _headers = req.request().clone();
            // println!("{:?}", req);
            println!("Hi from response");
            Ok(res)
        })
    }
}


// HeaderMap { inner: {"host": Value { inner: ["127.0.0.1:8000"] }, "accept": Value { inner: ["*/*"] }, "postman-token": Value { inner: ["660d7c3e-2461-4bbd-8839-cadd5c39cc73"] }, "connection": Value { inner: ["keep-alive"] }, "user-agent": Value { inner: ["PostmanRuntime/7.30.1"] }, "accept-encoding": Value { inner: ["gzip, deflate, br"] }} }