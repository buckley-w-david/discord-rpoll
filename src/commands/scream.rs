use serenity::client::Context;
use serenity::framework::standard::{
    macros::command,
    CommandResult,
};
use serenity::model::channel::Message;

use tracing::{error, info};

use rand::{self, Rng};
use rand::seq::SliceRandom;

#[derive(Debug)]
struct VariableScream<'a> {
    static_prefix: &'a str,
    variable_pattern: &'a str,
    static_postfix: &'a str,
}

#[derive(Debug)]
enum Scream<'a> {
    Static(&'a str),
    Variable(VariableScream<'a>),
}

const SCREAMS: &[Scream] = &[
    Scream::Static("😱"),
    Scream::Static("🙀"),
    Scream::Static("Eek!"),
    Scream::Static("Gahh!"),
    Scream::Variable(VariableScream {
        static_prefix: "AAA", variable_pattern: "H", static_postfix: "!",
    }),
];

#[command]
async fn scream(ctx: &Context, msg: &Message) -> CommandResult {
    let scream = SCREAMS.choose(&mut rand::thread_rng()).unwrap();

    info!("Screaming with {:#?}", scream);

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            match scream {
                Scream::Static(text) => m.content(text),
                Scream::Variable(var) => {
                    let mut scream = String::from(var.static_prefix);
                    for _i in 0..rand::thread_rng().gen_range(3, 20) {
                        scream.push_str(var.variable_pattern);
                    }
                    scream.push_str(var.static_postfix);

                    m.content(scream)
                },
            };
            m
        })
        .await;

    if let Err(why) = msg {
        error!("Unable to send message: {}", why);
    }

    Ok(())
}
