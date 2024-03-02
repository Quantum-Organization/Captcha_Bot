use super::{Data, image::draw_text};
use serenity::all::{
    ButtonStyle, Colour, ComponentInteraction, Context, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionResponseFlags, Timestamp
};

pub async fn run(ctx: Context, interaction: ComponentInteraction) {
    let mut data = ctx.data.write().await;
    let codes = data.get_mut::<Data>().unwrap();

    let code = rand::random::<u32>()
        .to_string()
        .chars()
        .take(6)
        .collect::<String>();

    let generate_image = draw_text(&code);
    
    if codes.contains_key(&interaction.user.id.get().clone()) {
        codes.remove(&interaction.user.id.get().clone());
    }
    
    codes.insert(interaction.user.id.get().clone(), code);

    let attachment = CreateAttachment::bytes(generate_image, "captcha.png");
    let footer = CreateEmbedFooter::new("Quantum Codes - Captcha");

    let embed = CreateEmbed::default()
        .title("Captcha")
        .description("Digite o texto que você vê na imagem abaixo")
        .image("attachment://captcha.png")
        .color(Colour::DARK_GREY)
        .timestamp(Timestamp::now())
        .footer(footer);
    

    let button = CreateButton::new("solve_captcha")
        .label("Resolver Captcha")
        .style(ButtonStyle::Secondary);

    let action_row = CreateActionRow::Buttons(vec![button]);

    let message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .files(vec![attachment])
        .flags(InteractionResponseFlags::EPHEMERAL)
        .components(vec![action_row]);

    let reply = CreateInteractionResponse::Message(message);
    interaction.create_response(&ctx.http, reply).await.unwrap()
}
