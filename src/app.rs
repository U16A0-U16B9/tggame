use std::future::Future;
use teloxide::prelude::*;
use crate::services::bootstrap;
use crate::commands::{answer, Command};

pub fn init() -> impl Future {
    bootstrap::start();
    let bot = Bot::from_env();
    Command::repl(bot, answer)
    // teloxide::repl(bot, |bot: Bot, message: Message| async move {
    //     respond(())
    // })
}