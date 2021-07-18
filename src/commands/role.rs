use serenity::client::Context;
use serenity::framework::standard::{
    Args,
    macros::command,
    CommandResult,
};
use serenity::model::{
    channel::{Message, ReactionType},
    guild,
};
use tracing::{error, info};

enum RequestType {
    Add,
    Remove,
    Colour,
}

#[command]
async fn role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = guild::Guild::get(&ctx.http, msg.guild_id.unwrap()).await.unwrap();
    let mut member = guild.member(&ctx.http, msg.author.id).await.unwrap();

    let request_type = match args.single::<String>().unwrap().as_ref() {
        "add" | "a" => RequestType::Add,
        "remove" | "r" => RequestType::Remove,
        "colour" | "c" => RequestType::Colour,
        _ => {
            error!("message \"{}\" does not specify request type", msg.content);
            return Ok(());
        }
    };

    match request_type { 
            RequestType::Add | RequestType::Remove => {
                let roles_to_change = args.iter::<String>()
                    .map(|role_name| guild.role_by_name(&role_name.unwrap()).unwrap())
                    .collect::<Vec<_>>();

                for role in roles_to_change {
                    let result = match request_type {
                        RequestType::Add => member.add_role(&ctx.http, role.id).await,
                        RequestType::Remove => member.remove_role(&ctx.http, role.id).await,
                        _ => Ok(()),
                    };
                    let react = if let Ok(_) = result {
                        "👍"
                    } else {
                        "👎"
                    };
                    let result = msg.react(&ctx.http, ReactionType::Unicode(String::from(react))).await;
                    info!("result: {:#?}", result);
                }
            },
            RequestType::Colour => {
                let role_name = format!("colour-{}", msg.author.id.as_u64());
                let red = args.single::<u32>().unwrap();
                let green = args.single::<u32>().unwrap();
                let blue = args.single::<u32>().unwrap();

                let pos = guild.roles.len() as u8 - 1;

                let colour = red << 16 | green << 8 | blue;
                info!("colour requested is {}", colour);
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
            }
    }; 

    Ok(())
}
