use teloxide::prelude::Message;
use teloxide::types::User;

pub fn get_user_from_message(message: &Message) -> User {
    message.from().expect("User not found").clone()
}
