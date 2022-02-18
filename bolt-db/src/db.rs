use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use sqlx::query;

#[derive(Clone)]
pub struct DbConn {
   pool: PgPool
}

pub async fn initdb() -> Result<DbConn, sqlx::error::BoxDynError> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await?;
    Ok(DbConn{pool})
}

impl DbConn {
    pub async fn insert_val(&self, i: i32, v: &str) -> Result<(), sqlx::error::BoxDynError> {
        query!(r#"insert into test values($1, $2)"#, i, v).execute(&self.pool).await?;
        Ok(())
    }

}