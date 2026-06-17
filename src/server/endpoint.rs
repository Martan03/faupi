use crate::specs::{body::body::Body, response::Response};

#[derive(Debug)]
pub struct Endpoint {
    pub request: Option<Body>,
    pub response: Response,
}

impl Endpoint {
    /// Creates new endpoint containing given response and empty request.
    pub fn new(response: Response) -> Self {
        Self {
            request: None,
            response,
        }
    }

    /// Sets the endpoint's request template to given value.
    pub fn request<T>(mut self, request: T) -> Self
    where
        T: Into<Option<Body>>,
    {
        self.request = request.into();
        self
    }
}
