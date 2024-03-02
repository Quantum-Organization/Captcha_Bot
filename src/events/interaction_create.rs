use super::{commands, components};
use serenity::all::{Context, Interaction};

pub async fn run(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => {
            match command.data.name.as_str() {
                "setup" => commands::setup::run(ctx, command).await,
                _ => println!("❌ - Command not found!"),
            }
        },
        Interaction::Component(component) => {
            match component.data.custom_id.as_str() {
                "verify" => components::verify::run(ctx, component).await,
                "solve_captcha" => components::solve_captcha::run(ctx, component).await,
                _ => println!("❌ - Component not found!"),
            }
        },
        Interaction::Modal(component) => {
            match component.data.custom_id.as_str() {
                "captcha_modal" => components::captcha_modal::run(ctx, component).await,
                _ => println!("❌ - Modal not found!")
            }
        }
        _ => println!("🔘 - Some other interaction detected!"),
    }
}
