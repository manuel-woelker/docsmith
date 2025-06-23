use error_stack::{Context, Report};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct DocsmithError(pub Report<DocsmithErrorContext>);

pub struct DocsmithErrorContext {
    pub context: Box<dyn Context>,
}

impl Debug for DocsmithError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl DocsmithErrorContext {
    #[track_caller]
    pub fn new<T: Context>(context: T) -> DocsmithErrorContext {
        DocsmithErrorContext {
            context: Box::new(context),
        }
    }

    #[track_caller]
    pub fn from_string<T: Into<String>>(message: T) -> DocsmithErrorContext {
        DocsmithErrorContext {
            context: Box::new(GeneralError {
                message: message.into(),
            }),
        }
    }
}

pub struct BoxedErrorContext {
    pub error: BoxedError,
}

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

impl BoxedErrorContext {
    #[track_caller]
    pub fn new(error: BoxedError) -> BoxedErrorContext {
        BoxedErrorContext { error }
    }
}

impl Display for BoxedErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}

impl Debug for BoxedErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.error, f)
    }
}

impl Error for BoxedErrorContext {}

impl DocsmithErrorContext {
    pub fn from_context<T: Context>(context: T) -> DocsmithErrorContext {
        DocsmithErrorContext {
            context: Box::new(context),
        }
    }
}

impl Display for DocsmithErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.context, f)
    }
}

impl Debug for DocsmithErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.context, f)
    }
}

impl Error for DocsmithErrorContext {}

pub struct GeneralError {
    pub message: String,
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.message, f)
    }
}

impl Debug for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.message, f)
    }
}

impl Error for GeneralError {}

impl Display for DocsmithError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl DocsmithError {
    #[track_caller]
    pub fn change_context<S: Into<String>>(self, message: S) -> Self {
        Self(
            self.0
                .change_context(DocsmithErrorContext::from_string(message.into())),
        )
    }
}

impl DocsmithError {
    #[track_caller]
    pub fn new<T: Context>(error: T) -> DocsmithError {
        DocsmithError(Report::new(DocsmithErrorContext::from_context(error)))
    }

    #[track_caller]
    pub fn from_string<T: Into<String>>(message: T) -> DocsmithError {
        DocsmithError(Report::new(DocsmithErrorContext::from_string(message)))
    }

    #[track_caller]
    pub fn from_boxed(error: BoxedError) -> DocsmithErrorContext {
        DocsmithErrorContext {
            context: Box::new(BoxedErrorContext::new(error)),
        }
    }
}

impl<T: Context> From<T> for DocsmithError {
    #[track_caller]
    fn from(error: T) -> Self {
        Self::new(error)
    }
}

#[macro_export]
macro_rules! bail {
    ($($args:tt)+) => {
        return Err($crate::err!($($args)+))
    }
}

#[macro_export]
macro_rules! err {
    ($($args:tt)+) => {
        $crate::error::DocsmithError::from_string(format!($($args)+))
    };
}
