use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use crate::{Api, ApiError};

#[derive(Debug, Clone)]
pub struct GustGraphQl;

impl GustGraphQl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GustGraphQl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
pub trait GustApiServer {
    async fn serve_api(
        &self,
        pool: Pool<ConnectionManager<SqliteConnection>>,
        api: Api,
    ) -> Result<(), ApiError>;
}

#[async_trait::async_trait]
impl GustApiServer for GustGraphQl {
    async fn serve_api(
        &self,
        _pool: Pool<ConnectionManager<SqliteConnection>>,
        _api: Api,
    ) -> Result<(), ApiError> {
        Ok(())
    }
}
