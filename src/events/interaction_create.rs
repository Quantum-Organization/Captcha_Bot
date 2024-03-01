use serenity::all::{Context, Interaction};
use super::commands;

pub async fn run(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        match command.data.name.as_str() {
            "base" => commands::base::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        }
    }

}