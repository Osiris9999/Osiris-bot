use teloxide::{prelude::*, utils::command::BotCommand};
use std::error::Error;
use urlshortener::{client::UrlShortener, providers::Provider};
mod scrape;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows details about the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "cuss you with no mercy.")]
    Cuss,
    #[command(description = "Gets the source code link.")]
    Code,
    #[command(description = "Print random jokes.")]
    J,
    #[command(description = "Shorten a given url.")]
    Short(String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
Command::Start =>{
    
        cx.answer(format!("This is a bot made in rust by @I_am_Osiris9999. Type /help to see what this bot can do.")).await?
        }

Command::Help =>cx.answer(Command::descriptions()).await?,

Command::Cuss => {
        cx.answer(format!("Phak you bruh")).await?
        }

Command::Code => {
        cx.answer(format!("https://github.com/Osiris9999/Osiris-bot")).await?
        }
Command::J => {let j =  scrape::juke().await;
               cx.reply_to(format!("{:#?}", j)).await?
        }
Command::Short(link) => {
                let us = UrlShortener::new().unwrap();
                let short_url = us.generate(&link, &Provider::Rlu).unwrap();
                cx.answer(short_url).await?
 
}
};

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Osiris Bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "OSIRIS's BOT".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
