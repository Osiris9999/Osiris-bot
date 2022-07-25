use teloxide::{prelude::*, utils::command::BotCommands, types::ParseMode::Markdown};
use std::error::Error;
use urlshortener::{client::UrlShortener, providers::Provider};
mod scrape;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows details about the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Gets the source code link.")]
    Code,
    #[command(description = "Print random jokes.")]
    Jk,
    #[command(description = "Shorten a given url.")]
    Short(String),
}

// change code from here

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
Command::Start =>{
    
        bot.send_message(message.chat.id, format!("A bot made in rust by Osiris. Type /help to see what this bot can do.")).parse_mode(Markdown).await?
        }

Command::Help => {
bot.send_message(message.chat.id, format!("I am a bot made by [Osiris](https://t.me/Osiris_9999) in [rust](https://www.rust-lang.org/).
Here's a list of my commands:-
`/help` ~ _Display this text._
`/start` ~ _Shows bot info._ 
`/code` ~ _Gets the source code of bot._
`/jk` ~ _Print random jokes._
`/short [url]` ~ _Shorten a given url._")).disable_web_page_preview(true).parse_mode(Markdown).await?
        }

Command::Code => {
        bot.send_message(message.chat.id, format!("https://github.com/Osiris9999/Osiris-bot")).await?
        }
Command::Jk => {
        let j =  scrape::juke().await;
        bot.send_message(message.chat.id, j.unwrap()).await?
        }
Command::Short(link) => {
                let us = UrlShortener::new().unwrap();
                let short_url = us.generate(&link, &Provider::Rlu).unwrap();
                bot.send_message(message.chat.id, short_url).await?
 
}
};

    Ok(())
}
//end changes here

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Osiris Bot...");

     let bot = Bot::from_env().auto_send();


   // let bot_name: String = "Osiris9999_BOT".to_string();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}
