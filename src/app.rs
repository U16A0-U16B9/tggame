use crate::commands::tutorial_commands::handle_tutorial_callback;
use crate::commands::{answer, Command};
use crate::game::actions::handle_action_callback;
use crate::game::cheats::handle_cheats;
use crate::utility::string::parse_delimiter;
use log::error;
use std::error::Error;
use teloxide::prelude::*;

pub async fn init() {
    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn command_handler(bot: Bot, msg: Message, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    answer(bot, msg, command).await;
    Ok(())
}

async fn message_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    handle_cheats(bot, msg).await;
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = q.data.clone().expect("no callback data found");
    let (pred_delimiter, post_delimiter) = parse_delimiter(data.as_str());
    match pred_delimiter {
        "" => {
            error!("no callback data found")
        }
        "tutorial" => handle_tutorial_callback(post_delimiter, bot, q).await,
        "action" => handle_action_callback(post_delimiter, bot, q).await,
        &_ => {
            error!("no callback action found AAA")
        }
    }

    Ok(())
}
