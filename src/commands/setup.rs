use std::vec;

use serenity::all::{
    ButtonStyle, Colour, CommandInteraction, Context, CreateActionRow, CreateButton, CreateCommand,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
    InteractionResponseFlags,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let verify_button = CreateButton::new("verify")
        .style(ButtonStyle::Primary)
        .label("Verificar");

    let action_row = CreateActionRow::Buttons(vec![verify_button]);

    let embed = CreateEmbed::default()
        .color(Colour::DARK_GREY)
        .title("Precisa se verificar para ter acesso ao servidor")
        .description("Clique no botão abaixo para se verificar");

    let embed_builder = CreateMessage::new()
        .embed(embed)
        .components(vec![action_row]);

    command
        .channel_id
        .send_message(&ctx.http, embed_builder)
        .await
        .unwrap();

    let reply_data = CreateInteractionResponseMessage::new()
        .content("Setup complete!")
        .flags(InteractionResponseFlags::EPHEMERAL);

    let reply_builder = CreateInteractionResponse::Message(reply_data);

    command
        .create_response(&ctx.http, reply_builder)
        .await
        .unwrap();
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("Setup the captcha system.")
}
