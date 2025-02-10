use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnknownError {
    /// 其他自定义错误
    #[error("A specific error occurred: {0}")]
    UnknownError(String),
}
