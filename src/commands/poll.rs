use csv::ReaderBuilder;

use serenity::client::Context;
use serenity::framework::standard::{
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
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(msg.content.as_bytes());

    if let Some(Ok(result)) = rdr.records().next() {
        if result.len() < 3 {
            error!("message \"{}\" is not valid", msg.content);
            return Ok(());
        }

        let title = result.get(1).unwrap();

        let options = (2..result.len())
            .map(|i| result.get(i).unwrap())
            .enumerate()
            .map(|(i, option_text)| {
                let mut emoji = emoji_digit(i + 1);
                emoji.push_str("\t");
                emoji.push_str(option_text);
                emoji
            })
            .collect::<Vec<_>>()
            .join("\n");

        let msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field(title, options, false);
                    e
                });

                m
            })
            .await;

        if let Err(ref why) = msg {
            info!("Error sending message: {:?}", why);
        }

        let msg = msg.unwrap();
        for i in 2..result.len() {
            let react = msg
                .react(&ctx.http, ReactionType::Unicode(emoji_digit(i - 1)))
                .await;
            if let Err(why) = react {
                error!("Error sending reaction: {:?}", why);
            }
        }
    } else {
        error!("message \"{}\" is not valid", msg.content);
    };

    Ok(())
}
