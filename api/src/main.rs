use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::time::Instant;
use actix_web::rt::time::interval;
use rusty_interaction::Builder;
use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::application::ApplicationCommandInteractionData;
use rusty_interaction::types::components::{ComponentButtonBuilder, ComponentButtonStyle, ComponentRowBuilder};
use rusty_interaction::types::embed::{Embed, EmbedBuilder, EmbedFooter, EmbedImage};
use rusty_interaction::types::interaction::*;
use rusty_interaction::types::Snowflake;
use rusty_interaction::types::message::Message;
use image;

const PUB_KEY: &str = "d60cfa59f0d6b0ed79b2437ec22b3f2edd7b5414e59ae1578dd1211a1ff098b3";
const APP_ID: u64 = 937152370331242557;

// We defer in this instance, because we don't want to edit anything
#[rusty_interaction::component_handler]
#[defer]
async fn delete_button(ctx: Context) -> Result<InteractionResponse, Infallible> {
    if let Ok(_) = ctx.delete_original().await {}
    return ctx.respond().none();
}

#[rusty_interaction::slash_command]
async fn ping_handler(ctx: Context) -> Result<InteractionResponse, Infallible> {
    ctx.respond().content("Pong! I'm stateless, so I have no ping!").finish()
}

#[rusty_interaction::slash_command]
// #[rusty_interaction::defer]
async fn quote_handler(ctx: Context) -> Result<InteractionResponse, Infallible> {
    let interaction_data = ctx.interaction.data.as_ref().unwrap().resolved.as_ref().unwrap();
    let messages = interaction_data.messages.as_ref().unwrap();
    let mut message_content: &str = "";

    for (id, message) in messages.into_iter() {
        println!("{} : {:?}", id, message.content);
        message_content = message.content.as_ref().unwrap().as_str();
    }

    println!("generating image...?");
    let before_call = Instant::now();
    let url = image::generate_image(message_content).await;
    println!("total time elapsed: {:?}", before_call.elapsed().as_millis());
    println!("url: {:?}", url);

    let components = ComponentRowBuilder::default()
        .add_button(
            ComponentButtonBuilder::default()
                .label("Delete")
                .custom_id("DELETE")
                .style(&ComponentButtonStyle::Danger)
                .finish(),
        )
        .finish();

    let embed = EmbedBuilder::default()
        .description(format!("here's the message: `{}`", message_content))
        .color(0xc9a0dc as u32)
        .image(EmbedImage::default().url(url))
        .finish();

    ctx.respond()
        .add_component_row(components)
        .add_embed(&embed)
        .finish()
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);

    handle.add_global_command("ping", ping_handler);
    handle.add_global_command("quote", quote_handler);

    // Here we attach our custom ids we defined with our buttons to the handler
    handle.add_component_handle("DELETE", delete_button);

    handle.run(3000).await;
}
