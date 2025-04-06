use crate::models::errors::RepositoryError;

pub mod post;
pub mod user;

/**
A repository representing the interactions between Tyche and a table in its database.
- `T` - The type of entity stored in the table.
- `U` - The type of the input DTO used to create an entity.
- `V` - The type of the primary key for the table.
 */
pub trait Repository<T, U, V> {
    fn create(
        &self,
        input: U,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn read(
        &self,
        primary_key: V,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn update(
        &self,
        primary_key: V,
        input: U,
    ) -> impl std::future::Future<Output = Result<T, RepositoryError>> + Send;
    fn delete(
        &self,
        primary_key: V,
    ) -> impl std::future::Future<Output = Option<RepositoryError>> + Send;
}
