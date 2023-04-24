use crate::commands::{answer, Command};
use crate::services::bootstrap;
use std::future::Future;
use teloxide::prelude::*;

pub fn init() -> impl Future {
    let bot = Bot::from_env();
    Command::repl(bot, answer)
    // teloxide::repl(bot, |bot: Bot, message: Message| async move {
    //     respond(())
    // })
}
