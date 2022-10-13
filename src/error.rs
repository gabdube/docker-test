use std::{fmt, error::Error};

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum InnerServerErrorCode {
    Ok,
    FailedToLoadTemplate,
    FailedToCacheAsset,
}

impl Default for InnerServerErrorCode {
    fn default() -> Self {
        Self::Ok
    }
}

#[derive(Clone, Debug)]
pub struct InnerServerError {
    code: InnerServerErrorCode,
    msg: String,
}

impl InnerServerError {

    pub fn as_exit_code(&self) -> i32 {
        self.code as u32 as i32
    }

    pub fn failed_to_load_template<S: Into<String>>(msg: S) -> Self {
        InnerServerError {
            code: InnerServerErrorCode::FailedToLoadTemplate,
            msg: msg.into(),
        }
    }

    pub fn failed_to_cache_asset<S: Into<String>>(msg: S) -> Self {
        InnerServerError {
            code: InnerServerErrorCode::FailedToCacheAsset,
            msg: msg.into(),
        }
    }

}

impl fmt::Display for InnerServerError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E-{:?} {}", self.code, self.msg)
    }

}


impl Error for InnerServerError { }
