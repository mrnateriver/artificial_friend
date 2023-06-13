use std::time::{Duration, Instant};

use rand::seq::SliceRandom;

use crate::{
    dialog_message::{DialogEntry, DialogMessage},
    openai_client::OpenAiChatMessage,
};

pub struct DialogState {
    messages: Vec<DialogMessage>,
    last_message_received: Option<Instant>,
    is_participating: bool,
}

impl DialogState {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            last_message_received: None,
            is_participating: false,
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.last_message_received = None;
        self.is_participating = false;
    }

    pub fn add_message(&mut self, msg: DialogMessage) {
        self.messages.push(msg);
        self.last_message_received = Some(Instant::now());
    }

    pub fn set_participating(&mut self) {
        self.is_participating = true;
    }

    pub fn is_participating(&self) -> bool {
        self.is_participating
    }

    pub fn get_message(&self, msg_id: i32) -> Option<&DialogMessage> {
        self.messages.iter().find(|msg| msg.get_id() == msg_id)
    }

    pub fn get_duration_since_last_message(&self) -> Option<Duration> {
        self.last_message_received.map(|last_message_received| last_message_received.elapsed())
    }

    pub fn peek(&self) -> Option<&DialogMessage> {
        self.messages.last()
    }

    pub fn random_message(&self) -> Option<&DialogMessage> {
        self.messages.choose(&mut rand::thread_rng())
    }

    pub fn slice_up_to(&self, msg_id: i32) -> Self {
        Self {
            messages: self
                .messages
                .iter()
                .filter_map(|msg| if msg.get_id() <= msg_id { Some(msg.clone()) } else { None })
                .collect(),
            last_message_received: self.last_message_received,
            is_participating: self.is_participating,
        }
    }

    pub fn get_prompt_messages(&self) -> impl Iterator<Item = OpenAiChatMessage> + '_ {
        self.messages.iter().map(|msg| {
            let text: String = msg.get_text().into();

            if let Some(author_name) = msg.get_author_name() {
                OpenAiChatMessage::User {
                    name: author_name.into(),
                    text,
                }
            } else {
                OpenAiChatMessage::Assistant { text }
            }
        })
    }
}
