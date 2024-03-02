use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, InputTextStyle};

pub async fn run(ctx: Context, interaction: ComponentInteraction) {

    let input_text = CreateInputText::new(InputTextStyle::Short, "captcha", "captcha_code");
    let modal_component = CreateActionRow::InputText(input_text);
    let modal = CreateModal::new("captcha_modal", "Resolver Captcha").components(vec![modal_component]);

    let response = CreateInteractionResponse::Modal(modal);
    interaction.create_response(&ctx.http, response).await.unwrap()
    
}