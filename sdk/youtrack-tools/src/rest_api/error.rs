use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::Error),
    RestError (RestError),
    ConverterError(serde_json::Error),
}

#[derive(Debug)]
pub enum RestError{
    NotFound {
        id: String
    },
    Error {
        status_code: u16,
        message: String,
    },
    Conflict(String),
    EmptyList
}

impl Error {
    pub fn not_found(id: String) -> Self {
        Self::RestError(RestError::NotFound { id })
    }

    pub fn empty_list() -> Self {
        Self::RestError(RestError::EmptyList)
    }

    pub fn rest_error(status_code: u16, message:String)->Self {
        Self::RestError(RestError::Error {status_code, message})
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HttpError(e) => { e.fmt(f) }
            Error::RestError(RestError::NotFound { id }) => {
                // Error::RestError { status: 404u16, id: Some(id) } => {
                write!(f, "Entry with id: {} not found", id)
            }
            Error::ConverterError(e) => e.fmt(f),
            e @ Error::RestError { .. } => {
                write!(f, "Some rest error: {:?}", e)
            }
        }
    }
}

