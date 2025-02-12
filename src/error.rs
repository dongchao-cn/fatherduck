use derive_new::new;

use thiserror::Error;
use pgwire::api::{ClientInfo, ErrorHandler};
use pgwire::error::PgWireError;

#[derive(Error, Debug)]
pub enum UnknownError {
    /// 其他自定义错误
    #[error("A specific error occurred: {0}")]
    UnknownError(String),
}


#[derive(new, Debug)]
pub struct FatherDuckErrorHandler {
}

impl ErrorHandler for FatherDuckErrorHandler {
    fn on_error<C>(&self, _client: &C, _error: &mut PgWireError)
    where
        C: ClientInfo,
    {
        println!("on_error: {:?}", _error);
    }
}
