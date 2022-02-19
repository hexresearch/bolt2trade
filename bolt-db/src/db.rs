use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use sqlx::query;
use sqlx::error::BoxDynError;
use bolt_shared::types::User;
use sqlx::Row;
use sqlx::postgres::PgRow;

#[derive(Clone)]
pub struct DbConn {
   pool: PgPool
}

pub async fn initdb() -> Result<DbConn, BoxDynError> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await?;
    Ok(DbConn{pool})
}

impl DbConn {
    pub async fn insert_user(&self, linking_key: String, uname: Option<String>) -> Result<(), BoxDynError> {
        query!(r#"insert into users values ($1, $2) on conflict (user_id) do nothing"#, linking_key, uname)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user(&self, key: String) -> Result<Option<User>, BoxDynError> {
        let ou = sqlx::query_as(r#"select * from users where user_id=$1"#).bind(key)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ou)
    }

    pub async fn delete_user(&self, key: String) -> Result<(), BoxDynError> {
        query!(r#"delete from users where user_id=$1"#, key)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

fn parse_user(row: PgRow) -> Option<User>{
    let uid = row.try_get("user_id").ok()?;
    let user_name = row.try_get("user_name").ok()?;
    Some(User{uid, user_name})
}