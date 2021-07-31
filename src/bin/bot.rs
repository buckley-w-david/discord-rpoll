use std::{
    env,
    collections::HashSet,
};

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    prelude::*,
};

use tracing::error;
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

use sqlx::{
    sqlite::SqlitePool,
};

use utility_bot::{
    database,
    commands::{
        role::ROLE_COMMAND,
        poll::POLL_COMMAND,
        scream::SCREAM_COMMAND,
    }
};


struct Handler;

#[async_trait]
impl EventHandler for Handler { }

#[group]
#[commands(poll, role, scream)]
struct General;

#[group]
#[owners_only]
struct Owner;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    let db_conn = env::var("DATABASE_URL")
        .expect("Expected a db connection string in the environment");

    let http = Http::new_with_token(&token);
    let pool = match SqlitePool::connect(&db_conn).await {
        Ok(pool) => pool,
        Err(why) => panic!("Could not create a db pool: {:?}", why),
    };

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix("!"))
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<database::DbConnection>(pool);
    }

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
