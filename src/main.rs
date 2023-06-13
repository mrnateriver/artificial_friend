mod dialog_handler;
mod dialog_message;
mod dialog_state;
mod openai_client;

use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

use reqwest::Client;
use teloxide::prelude::*;
use tracing::{debug, info, Level};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{dialog_handler::DialogHandler, dialog_state::DialogState, openai_client::OpenAiClient};

#[tokio::main]
async fn main() {
    let filter = Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).expect("RUST_LOG should be a valid tracing filter");
    tracing_subscriber::fmt().with_max_level(Level::TRACE).json().finish().with(filter).init();

    info!("starting bot");

    let bot = Bot::from_env();

    let openai = Arc::new(OpenAiClient::new(
        Client::new(),
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY should be set"),
    ));

    let state = Arc::new(Mutex::new(DialogState::new()));

    let handler = Arc::new(DialogHandler::new(
        &std::env::var("CHARACTER_NAME").expect("CHARACTER_NAME must be set"),
        &std::env::var("CHARACTER_DESCRIPTION").expect("CHARACTER_DESCRIPTION must be set"),
        std::env::var("DIALOG_TIMEOUT")
            .ok()
            .map(|v| Duration::from_secs(v.parse().expect("DIALOG_TIMEOUT must be a valid number"))),
    ));

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let state = state.clone();
        let openai = openai.clone();
        let handler = handler.clone();

        async move {
            debug!("received message: {:?}", msg);

            handler.handle_message(&state, &openai, &bot, &msg).await;

            Ok(())
        }
    })
    .await;
}
