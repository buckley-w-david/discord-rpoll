/*
use crate::database::DbConnection;

use tokio::stream::StreamExt;

use serenity::client::Context;
use serenity::framework::standard::{
    macros::command,
    CommandResult,
};
use serenity::model::channel::Message;

use tracing::{error, info};

use sqlx::{self, Row};

// Commands for testing DB interaction

#[command]
#[owners_only]
async fn execute(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = DbConnection::from(ctx).await;

    let query_string = msg.content.split(" ").skip(1).collect::<Vec<&str>>().join(" ");
    info!("executing: {}", query_string);

    let rows = sqlx::query(&query_string).execute(&conn).await;

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("{:#?}", rows));
            m
        })
        .await;

    if let Err(ref why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[owners_only]
async fn fetch(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = DbConnection::from(ctx).await;

    let query_string = msg.content.split(" ").skip(1).collect::<Vec<&str>>().join(" ");
    info!("fetching: {}", query_string);

    let mut rows = sqlx::query(&query_string).fetch(&conn);

    while let Some(row) = rows.try_next().await? {
        let val: &str = row.try_get("test")?;
        let msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(format!("{:#?}", val));
                m
            })
            .await;

        if let Err(ref why) = msg {
            error!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}
*/
