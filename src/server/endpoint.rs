use crate::specs::{body::body::Body, response::EndpointResponse};

#[derive(Debug)]
pub struct Endpoint {
    pub request: Option<Body>,
    pub response: EndpointResponse,
}

impl Endpoint {
    /// Creates new endpoint containing given response and empty request.
    pub fn new(response: EndpointResponse) -> Self {
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
