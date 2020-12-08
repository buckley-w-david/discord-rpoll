use serenity::client::Context;
use serenity::framework::standard::{
    Args,
    macros::command,
    CommandResult,
};
use serenity::model::channel::{Message, ReactionType};

use tracing::{error, info};

fn emoji_digit(i: usize) -> String {
    match i {
        1 => String::from("1️⃣"),
        2 => String::from("2️⃣"),
        3 => String::from("3️⃣"),
        4 => String::from("4️⃣"),
        5 => String::from("5️⃣"),
        6 => String::from("6️⃣"),
        7 => String::from("7️⃣"),
        8 => String::from("8️⃣"),
        9 => String::from("9️⃣"),
        _ => String::from("0️⃣"),
    }
}

#[command]
async fn poll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    args.quoted();
    let title = args.single_quoted::<String>().unwrap();


    // Could not get args.iter() to respect quoted() even though docs say it should.
    // So I'm just iterating until the args are exhausted
    let mut options = Vec::new();
    let mut i = 0;
    while !args.is_empty() {
        let option_text = args.single_quoted::<String>().unwrap();
        let mut emoji = emoji_digit(i + 1);
        emoji.push_str("\t");
        emoji.push_str(&option_text);
        options.push(emoji);
        i += 1;
    }

    let joined = options.join("\n");

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field(title, joined, false);
                e
            });

            m
        })
        .await;

    if let Err(ref why) = msg {
        info!("Error sending message: {:?}", why);
    }

    let msg = msg.unwrap();
    for i in 1..options.len()+1 {
        let react = msg
            .react(&ctx.http, ReactionType::Unicode(emoji_digit(i)))
            .await;
        if let Err(why) = react {
            error!("Error sending reaction: {:?}", why);
        }
    };

    Ok(())
}
