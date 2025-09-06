use crate::args::serve_args::ServeArgs;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Serve(ServeArgs),
}
