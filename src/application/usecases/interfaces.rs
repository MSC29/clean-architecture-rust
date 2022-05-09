use crate::domain::error::ApiError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractUseCase {
    async fn execute<T>(&self) -> Result<T, ApiError>;
}
