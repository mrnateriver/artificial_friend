use std::{sync::Mutex, time::Duration};

use teloxide::{
    requests::{Request, Requester},
    types::{ChatKind, MediaKind, Message, MessageId, MessageKind},
    Bot,
};
use tracing::{error, info, warn};

use crate::{
    dialog_message::{DialogEntry, DialogMessage, MessageData},
    dialog_state::DialogState,
    openai_client::{OpenAiChatMessage, OpenAiClient},
};

pub struct DialogHandler {
    character_name: String,
    character_description: String,
    dialog_timeout: Duration,
}

impl DialogHandler {
    pub fn new(character_name: &str, character_desc: &str, dialog_timeout: Option<Duration>) -> Self {
        const DEFAULT_DIALOG_TIMEOUT: Duration = Duration::from_secs(3 * 60); // 3 minutes
        Self {
            character_name: character_name.to_owned(),
            character_description: character_desc.to_owned(),
            dialog_timeout: dialog_timeout.unwrap_or(DEFAULT_DIALOG_TIMEOUT),
        }
    }

    pub async fn handle_message(&self, state: &Mutex<DialogState>, openai: &OpenAiClient, bot: &Bot, msg: &Message) {
        if let ChatKind::Private(_) = msg.chat.kind {
            warn!(?msg.chat.kind, "received message from unsupported chat type");
            return;
        }

        let response_prompt = {
            let mut state = state.lock().unwrap();

            if self.is_dialog_timed_out(&state) {
                info!("previous dialog timed out");
                state.clear();
            }

            if let Some(dialog_message) = self.parse_message(&state, msg) {
                state.add_message(dialog_message);
            } else {
                error!(?msg, "received unsupported message");
            }

            self.should_respond(&state)
        };

        if let (Some(prompt), reply_to) = response_prompt {
            match self.get_message_response(openai, prompt.iter()).await {
                Ok(response) => {
                    info!("sending bot response: {}", response);

                    let mut to_send = bot.send_message(msg.chat.id, response);
                    to_send.reply_to_message_id = reply_to.map(MessageId);
                    let result = to_send.send().await;

                    match result {
                        Ok(msg) => {
                            let mut state = state.lock().unwrap();

                            if let Some(parsed_response) = self.parse_message(&state, &msg) {
                                state.add_message(parsed_response);
                                state.set_participating();
                            } else {
                                error!(?msg, "failed to parse own response message")
                            }
                        }
                        Err(err) => error!(?err, "failed to send response"),
                    }
                }
                Err(err) => error!(?err, "failed to determine response"),
            }
        }
    }

    fn is_dialog_timed_out(&self, state: &DialogState) -> bool {
        state.get_duration_since_last_message().map(|d| d > self.dialog_timeout).unwrap_or(false)
    }

    fn parse_message(&self, state: &DialogState, msg: &Message) -> Option<DialogMessage> {
        let reply = match &msg.kind {
            MessageKind::Common(body) => Some(body),
            _ => None,
        }
        .and_then(|b| b.reply_to_message.as_ref());

        self.parse_message_contents(msg).and_then(|c| {
            reply
                .and_then(|reply| state.get_message(reply.id.0))
                .map(|reply_message| DialogMessage::Reply {
                    message: c.clone(),
                    reply_to: MessageData::from_entry(reply_message),
                })
                .or(Some(DialogMessage::Standalone { message: c }))
        })
    }

    fn parse_message_contents(&self, msg: &Message) -> Option<MessageData> {
        let body = match &msg.kind {
            MessageKind::Common(body) => Some(body),
            _ => None,
        };

        let text = body.map(|b| &b.media_kind).and_then(|m| match m {
            MediaKind::Text(text) => Some(text.text.clone()),
            _ => None,
        });

        let author_name = body.and_then(|b| b.from.as_ref()).and_then(|a| a.username.clone());
        let is_bot = body.and_then(|b| b.from.as_ref()).map(|a| a.is_bot).unwrap_or(false);

        text.map(|text| MessageData::new(msg.id.0, text, author_name, is_bot))
    }

    async fn get_message_response(&self, openai: &OpenAiClient, prompt: impl Iterator<Item = &OpenAiChatMessage>) -> Result<String, ()> {
        info!("getting response from OpenAI");

        // TODO: extract constants into env vars
        let response = openai.chat(prompt, 200, 0.9, 1.5).await;

        match response {
            Ok(response) => {
                info!(?response, "got response from OpenAI");
                Ok(response)
            }
            Err(err) => {
                error!(?err, "failed to get response from OpenAI");
                Err(())
            }
        }
    }

    /// Returns AI prompt, or None if response is not needed
    fn should_respond(&self, state: &DialogState) -> (Option<Vec<OpenAiChatMessage>>, Option<i32>) {
        let last_message = state.peek();

        let mut messages_iter = None;
        let mut message_id: Option<i32> = None;

        if let Some(last_message) = last_message {
            let contains = last_message.get_text().to_lowercase().contains(&self.character_name.to_lowercase());
            let is_participating = state.is_participating();

            if match last_message {
                DialogMessage::Reply { reply_to, .. } => reply_to.is_bot(),
                _ => is_participating || contains,
            } {
                info!(?is_participating, mentioned = contains, mention = "replying to dialog");

                messages_iter = Some(state.get_prompt_messages());
                message_id = Some(last_message.get_id());
            }

            // TODO: make arbitrary interaction less intrusive
            // const ARBITRARY_RESPONSE_PROBABILITY: f32 = 0.3;
            // if rand::random::<f32>() < ARBITRARY_RESPONSE_PROBABILITY {
            //     let random_message = state.random_message();
            //     if let Some(random_message) = random_message {
            //         info!(?random_message, "replying to random message");
            //         return (
            //             Some(state.slice_up_to(random_message.get_id()).get_prompt_messages().collect()),
            //             Some(random_message.get_id()),
            //         );
            //     }
            // }
        }

        (
            messages_iter.map(|iter| {
                vec![OpenAiChatMessage::System {
                    text: self.character_description.clone(),
                }]
                .into_iter()
                .chain(iter)
                .collect()
            }),
            message_id,
        )
    }
}
