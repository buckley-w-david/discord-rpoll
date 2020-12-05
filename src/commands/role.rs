use csv::ReaderBuilder;

use serenity::client::Context;
use serenity::framework::standard::{
    macros::command,
    CommandResult,
};
use serenity::model::{
    channel::Message,
    guild,
};
use tracing::{error, info};

enum RequestType {
    Add,
    Remove,
    Colour,
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

        let guild = guild::Guild::get(&ctx.http, msg.guild_id.unwrap()).await.unwrap();
        let mut member = guild.member(&ctx.http, msg.author.id).await.unwrap();

        let request_type = match result.get(1).unwrap() {
            "add" | "a" => RequestType::Add,
            "remove" | "r" => RequestType::Remove,
            "colour" | "c" => RequestType::Colour,
            _ => RequestType::Unknown
        };

        if let RequestType::Unknown = request_type {
            error!("message \"{}\" does not specify request type", msg.content);
            return Ok(());
        }

        // TODO unwrap -> actual error handling

        match request_type { 
                RequestType::Add | RequestType::Remove => {
                    let roles_to_change = (2..result.len())
                        .map(|i| result.get(i).unwrap())
                        .map(|role_name| guild.role_by_name(role_name).unwrap())
                        .collect::<Vec<_>>();

                    for role in roles_to_change {
                        let result = match request_type {
                            RequestType::Add => member.add_role(&ctx.http, role.id).await,
                            RequestType::Remove => member.remove_role(&ctx.http, role.id).await,
                            _ => Ok(()),
                        };
                        info!("result: {:#?}", result);
                    }
                },
                RequestType::Colour => {
                    let role_name = format!("colour-{}", msg.author.id.as_u64());
                    let red = result.get(2).unwrap().parse::<u32>().unwrap();
                    let green = result.get(3).unwrap().parse::<u32>().unwrap();
                    let blue = result.get(4).unwrap().parse::<u32>().unwrap();

                    let pos = guild.roles.len() as u8 - 1;

                    let colour = red << 16 | green << 8 | blue;
                    info!("colour requested is {}", colour);
                    info!("Current roles {:#?}", guild.roles);
                    let id = if let Some(role) = guild.role_by_name(&role_name) {
                        info!("Existing role: {:#?}", role);
                        Some(role.id)
                    } else {
                        match guild.create_role(&ctx.http,|r| r.hoist(false).position(pos).mentionable(false).name(role_name).colour(colour as u64)).await {
                            Ok(role) => {
                                info!("New created is: {:#?}", role);
                                Some(role.id)
                            },
                            Err(ref why) => {
                                error!("Could not create roll: {}", why);
                                None
                            }
                        }
                    };

                    if let Some(id) = id {
                        let result = guild.id.edit_role(&ctx.http, id, |r| r.position(pos).colour(colour as u64)).await;
                        let add = member.add_role(&ctx.http, id).await;
                        info!("Edited Role: {:#?} {:#?}", result, add);
                    };
                    // if they don't => create one, else get it
                    // set the colour to the one in the request
                },
                RequestType::Unknown => (),
        };
    } else {
        error!("message \"{}\" is not valid", msg.content);
    };

    Ok(())
}
