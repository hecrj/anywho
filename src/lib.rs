use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Error(Arc<dyn std::error::Error + Send + Sync + 'static>);

impl Error {
    pub fn msg(error: impl Into<Cow<'static, str>>) -> Self {
        #[derive(Debug)]
        struct ErrorMsg(Cow<'static, str>);

        impl std::error::Error for ErrorMsg {
            fn description(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for ErrorMsg {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        Self(Arc::new(ErrorMsg(error.into())))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<dyn std::error::Error + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        &self.0
    }
}

impl std::ops::Deref for Error {
    type Target = dyn std::error::Error + Sync + Send;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(error: T) -> Self {
        Self(Arc::new(error))
    }
}

/// ```
/// use anywho::{anywho, Error};
///
/// let a = 1;
/// let b = 2;
///
/// assert_eq!(anywho!("{a} + {} = 3", b).to_string(), Error::msg("1 + 2 = 3").to_string());
/// ```
#[macro_export]
macro_rules! anywho {
    ($($arg:tt)*) => {
        $crate::Error::msg(std::fmt::format(std::format_args!($($arg)*)))
    };
}
