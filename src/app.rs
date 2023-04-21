use std::future::Future;
use teloxide::prelude::*;
use crate::services::bootstrap;

pub fn init() -> impl Future {
    bootstrap::start();
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, message: Message| async move {

        respond(())
    })
}