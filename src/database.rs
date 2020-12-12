use serenity::prelude::TypeMapKey;
use serenity::client::Context;

use sqlx::sqlite::SqlitePool;

pub struct DbConnection;
impl TypeMapKey for DbConnection {
    type Value = SqlitePool;
}

impl DbConnection {
    pub async fn from(ctx: &Context) -> SqlitePool {
        let data_read = ctx.data.read().await;
        data_read.get::<DbConnection>().expect("Expected DbConnection in TypeMap.").clone()
    }
}
