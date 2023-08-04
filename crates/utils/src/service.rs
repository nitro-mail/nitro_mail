use crate::Config;

pub trait Service: Send + Sync + 'static {
    type ServiceConfig: Config;
    type ServiceError: std::error::Error + Send + Sync + 'static;
}
