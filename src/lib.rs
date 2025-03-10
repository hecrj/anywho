use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Error(Arc<dyn std::error::Error + Send + Sync + 'static>);

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
