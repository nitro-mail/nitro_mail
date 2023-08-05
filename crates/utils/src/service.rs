use crate::configs::Config;
use async_trait::async_trait;
use auto_impl::auto_impl;

#[auto_impl(&, &mut, Box, Arc)]
pub trait Service: Send + Sync + 'static {
    type ServiceConfig: Config;
    type ServiceError: std::error::Error + Send + Sync + 'static;
}

/// Used in the MailService to get a new instance of the service.
///
/// # Example Implementation
/// ```rust
/// use std::future::Future;
/// use std::pin::Pin;
/// use utils::service::ServiceAccess;
/// use utils::service::tests::TestService;
/// #[derive(Clone)]
/// pub struct SimpleServiceAccess;
///
/// impl ServiceAccess for SimpleServiceAccess {
///     type ServiceResponse = TestService;
///     type Error = std::io::Error;
///     type Future = Pin<
///         Box<
///             dyn Future<Output = Result<Self::ServiceResponse, Self::Error>>
///                 + Send
///                 + 'static,
///         >,
///     >;
///
///     fn get_service(&self) -> Self::Future {
///         Box::pin(async move { TestService::load().await })
///     }
/// }
///```
///
/// # Example Implementation with Clone
///```rust
/// use std::convert::Infallible;
/// use futures::future::Ready;
/// use utils::service::{Service, ServiceAccess};
///
///     #[derive(Clone)]
///     pub struct SimpleService;
///
///     impl Service for SimpleService {
///         type ServiceConfig = ();
///         type ServiceError = Infallible;
///     }
///    impl ServiceAccess for SimpleService {
///        type ServiceResponse = SimpleService;
///        type Error = Infallible;
///        type Future = Ready<Result<Self::ServiceResponse, Self::Error>>;
///
///        fn get_service(&self) -> Self::Future {
///            futures::future::ready(Ok(self.clone()))
///        }
///    }
/// ```
pub trait ServiceAccess: Clone + Send + Sync + 'static {
    type ServiceResponse: Service;
    type Error: std::error::Error + Send + Sync + 'static;

    /// Some services use Clone to get a new instance of the service.
    /// Meaning that they do not need a future to get the service.
    /// So they can use a [futures::future::Ready](https://docs.rs/futures/latest/futures/future/struct.Ready.html) to return the service.
    type Future: std::future::Future<Output = Result<Self::ServiceResponse, Self::Error>>
        + 'static
        + Send;
    fn get_service(&self) -> Self::Future;
}

#[cfg(test)]
pub mod tests {
    #[derive(Clone)]
    pub struct TestService;

    impl super::Service for TestService {
        type ServiceConfig = ();
        type ServiceError = std::io::Error;
    }
    impl TestService {
        pub async fn load() -> Result<Self, std::io::Error> {
            Ok(Self)
        }
    }
}
