#[derive(Clone, Debug)]
#[derive(sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub uid: String,
    pub user_name: Option<String>
}