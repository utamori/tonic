pub struct ConnectLayer {
    _priv: (),
}

impl ConnectLayer {
    pub fn new() -> ConnectLayer {
        Self { _priv: () }
    }
}

impl Default for ConnectLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for ConnectLayer
where
    S: Service<http::Request<hyper::Body>, Response = http::Response<BoxBody>>,
    S: Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError> + Send,
{
    type Service = ConnectService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConnectService::new(inner)
    }
}
