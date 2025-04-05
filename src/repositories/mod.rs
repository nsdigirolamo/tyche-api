use crate::models::errors::RepositoryError;

pub mod post;
pub mod user;

pub trait Repository<T, U> {
    fn create(
        &self,
        input: U,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn read(
        &self,
        id: uuid::Uuid,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn update(
        &self,
        id: uuid::Uuid,
        input: U,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn delete(
        &self,
        id: uuid::Uuid,
    ) -> impl std::future::Future<Output = Option<RepositoryError>> + Send;
}
