// This is proof of concept code to make the owner of the servers's name rotate through
// colours. It's sectioned off and not used currently as it runs into discord rate limiting within
// about 20 minutes
//
use std::{
    sync::{Arc, atomic::{Ordering, AtomicBool}}
};

use serenity::{
    prelude::*,
    model::{id::GuildId, guild},
}

async fn rainbow(ctx: Arc<Context>, guild: guild::PartialGuild) {
    let user_id = guild.owner_id.as_u64();
    let role_name = format!("colour-{}", user_id);

    match guild.role_by_name(&role_name) {
        Some(role) => {
            let (r, g, b) = role.colour.tuple();

            let buffer = [r, g, b];

            let rgb: Srgb = Srgb::from_raw(&buffer).into_format();
            let hue_shifted = Hsl::from(rgb).shift_hue(4.0);
            let raw: [u8; 3] = Srgb::from(hue_shifted).into_format().into_raw();

            let (red, green, blue) = (raw[0] as u32, raw[1] as u32, raw[2] as u32);
            
            let colour = red << 16 | green << 8 | blue;
            let _result = guild.id.edit_role(&ctx.http, role.id, |r| r.colour(colour as u64)).await;
        },
        None => {
            error!("Could not get role with name: {}", role_name);
        }
    }
}
