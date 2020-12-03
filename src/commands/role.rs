use csv::ReaderBuilder;

use serenity::client::Context;
use serenity::framework::standard::{
    macros::command,
    CommandResult,
};
use serenity::model::{
    channel::Message,
    guild
};
use tracing::{error, info};

enum RequestType {
    Add,
    Remove,
    Unknown,
}

#[command]
async fn role(ctx: &Context, msg: &Message) -> CommandResult {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(msg.content.as_bytes());

    if let Some(Ok(result)) = rdr.records().next() {
        if result.len() < 3 {
            error!("message \"{}\" is not valid", msg.content);
            return Ok(());
        }

        // TODO unwrap -> actual error handling
        let guild = guild::Guild::get(&ctx.http, msg.guild_id.unwrap()).await.unwrap();
        let mut member = guild.member(&ctx.http, msg.author.id).await.unwrap();

        let request_type = match result.get(1).unwrap() {
            "add" => RequestType::Add,
            "a" => RequestType::Add,
            "remove" => RequestType::Remove,
            "r" => RequestType::Remove,
            _ => RequestType::Unknown
        };

        if let RequestType::Unknown = request_type {
            error!("message \"{}\" does not specify request type", msg.content);
            return Ok(());
        }

        let roles_to_change = (2..result.len())
            .map(|i| result.get(i).unwrap())
            .map(|role_name| guild.role_by_name(role_name).unwrap())
            .collect::<Vec<_>>();

        for role in roles_to_change {
            let result = match request_type {
                RequestType::Add => member.add_role(&ctx.http, role.id).await,
                RequestType::Remove => member.remove_role(&ctx.http, role.id).await,
                RequestType::Unknown => member.add_role(&ctx.http, role.id).await,
            };
            info!("result: {:#?}", result);
        }
    } else {
        error!("message \"{}\" is not valid", msg.content);
    };

    Ok(())
}
