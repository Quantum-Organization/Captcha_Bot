use super::Data;
use serenity::all::{
    ActionRow, ActionRowComponent, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, InteractionResponseFlags, ModalInteraction, RoleId,
};

pub async fn run(ctx: Context, interaction: ModalInteraction) {
    let mut data = ctx.data.write().await;
    let codes = data.get_mut::<Data>().unwrap();

    let code = codes.get(&interaction.user.id.get().clone());

    if code.is_none() {
        let message = CreateInteractionResponseMessage::new()
            .content("Captcha não encontrado, tente criar um novo!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply = CreateInteractionResponse::Message(message);

        interaction.create_response(&ctx.http, reply).await.unwrap();

        return;
    }

    let code = code.unwrap();

    let input_code = get_text(&interaction.data.components[0]);

    if input_code == *code {
        let member = interaction
            .guild_id
            .unwrap()
            .member(&ctx.http, interaction.user.id)
            .await
            .unwrap();

        let role = RoleId::new(1212488321440153634);
        member.add_role(&ctx.http, role).await.unwrap();

        let message = CreateInteractionResponseMessage::new()
            .content("Captcha resolvido com sucesso!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply = CreateInteractionResponse::Message(message);

        interaction.create_response(&ctx.http, reply).await.unwrap()
    } else {
        let message = CreateInteractionResponseMessage::new()
            .content("Captcha incorreto, crie um novo!")
            .flags(InteractionResponseFlags::EPHEMERAL);

        let reply = CreateInteractionResponse::Message(message);

        interaction.create_response(ctx.http, reply).await.unwrap()
    }

    codes.remove(&interaction.user.id.get().clone());
}

fn get_text(data: &ActionRow) -> String {
    let mut text = String::new();

    for component in &data.components {
        match component {
            ActionRowComponent::InputText(input_text) => {
                text.push_str(&input_text.value.as_ref().unwrap());
            }
            _ => {}
        }
    }

    text
}
