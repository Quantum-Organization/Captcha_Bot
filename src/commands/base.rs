use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionResponseFlags, Timestamp};


pub async fn run(ctx: Context, command: CommandInteraction) {

  let embed = CreateEmbed::default()
    .title("Bem vindos :D")
    .timestamp(Timestamp::now());

  let data = CreateInteractionResponseMessage::new().embed(embed).flags(
    InteractionResponseFlags::EPHEMERAL
  );
  
  let builder = CreateInteractionResponse::Message(data);
  command.create_response(&ctx.http, builder).await.unwrap();


}

pub fn register() -> CreateCommand {
    CreateCommand::new("base").description("A base command")
}