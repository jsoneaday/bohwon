use derive_more::{Error, Display};

#[derive(Error, Display, Debug)]
pub enum LiteseedError {
    InternalServer,
    ParseFloatError
}