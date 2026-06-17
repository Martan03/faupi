use crate::specs::body::body::Body;

/// Adds type constraint to the Body
///
/// It allows easier handling of type constraint values.
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct TypeConstraint {
    pub exp_type: String,
    pub value: Option<Box<Body>>,
}

impl TypeConstraint {
    /// Creates new [`TypeConstraint`] with given type and value
    pub fn new<T>(typ: &str, value: T) -> Self
    where
        T: Into<Option<Box<Body>>>,
    {
        Self {
            exp_type: typ.to_string(),
            value: value.into(),
        }
    }
}

impl Default for TypeConstraint {
    fn default() -> Self {
        Self {
            exp_type: "any".to_string(),
            value: Default::default(),
        }
    }
}
