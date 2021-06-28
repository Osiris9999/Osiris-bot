use teloxide::{prelude::*, utils::command::BotCommand, types::ParseMode::Markdown};
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
    Jk,
    #[command(description = "Shorten a given url.")]
    Short(String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
Command::Start =>{
    
        cx.reply_to(format!("A bot made in rust by Osiris. Type /help to see what this bot can do.")).parse_mode(Markdown).await?
        }

Command::Help => {
cx.reply_to(format!("I am a bot made by [Osiris](https://t.me/I_am_Osiris9999) in [rust](https://www.rust-lang.org/).
Here's a list of my commands:-
`/help` ~ _Display this text._
`/start` ~ _Shows bot info._ 
`/cuss` ~ _Cuss at you with no mercy._
`/code` ~ _Gets the source code of bot._
`/jk` ~ _Print random jokes._
`/short [url]` ~ _Shorten a given url._")).disable_web_page_preview(true).parse_mode(Markdown).await?
        }

Command::Cuss => {
        cx.answer(format!("Phak you bruh")).await?
        }

Command::Code => {
        cx.answer(format!("https://github.com/Osiris9999/Osiris-bot")).await?
        }
Command::Jk => {
        let j =  scrape::juke().await;
        cx.reply_to(j.unwrap()).await?
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

    let bot_name: String = "Osiris9999_BOT".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
