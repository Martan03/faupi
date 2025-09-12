use crate::args::{import::Import, serve::Serve};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Serve(Serve),
    Import(Import),
}
