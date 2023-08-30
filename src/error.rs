use std::fmt::Display;

use openssl::error::ErrorStack;

use crate::Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cause;
        if self.io_error.is_some() {
            cause = self.io_error.as_ref().unwrap().to_string();
        } else if self.ssl_error.is_some() {
            cause = self.ssl_error.as_ref().unwrap().to_string();
        } else {
            cause = String::from("unknown");
        }
        f.debug_struct("Error")
            .field("message", &self.message)
            .field("cause", &cause)
            .finish()
    }
}

impl Error {
    /// IO
    pub(crate) fn io(error: std::io::Error) -> Self {
        Self {
            message: String::from("IO Error"),
            io_error: Some(error),
            ssl_error: None,
        }
    }

    /// SSL
    pub(crate) fn ssl(error: ErrorStack) -> Self {
        Self {
            message: String::from("SSL Error"),
            io_error: None,
            ssl_error: Some(error),
        }
    }

    /// Custom error (only message)
    pub(crate) fn custom(message: &str) -> Self {
        Self {
            message: String::from(message),
            io_error: None,
            ssl_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use openssl::error::ErrorStack;

    use crate::Error;

    #[test]
    fn test_io() {
        let error = Error::io(std::io::Error::from(ErrorKind::AddrInUse));
        assert_eq!("IO Error", error.message);
        assert!(error.io_error.is_some());
        assert!(error.ssl_error.is_none());
    }

    #[test]
    fn test_ssl() {
        let error = Error::ssl(ErrorStack::get());
        assert_eq!("SSL Error", error.message);
        assert!(error.io_error.is_none());
        assert!(error.ssl_error.is_some());
    }
}
