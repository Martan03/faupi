use crate::args::serve::Serve;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Serve(Serve),
}
