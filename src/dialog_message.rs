pub trait DialogEntry {
    fn get_id(&self) -> i32;
    fn get_text(&self) -> &str;
    fn get_author_name(&self) -> Option<&str>;
    fn is_bot(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct MessageData {
    id: i32,
    text: String,
    author_name: Option<String>,
    is_bot: bool,
}

impl MessageData {
    pub fn new(id: i32, text: String, author_name: Option<String>, is_bot: bool) -> Self {
        Self {
            id,
            text,
            author_name,
            is_bot,
        }
    }

    pub fn from_entry(entry: &impl DialogEntry) -> Self {
        Self {
            id: entry.get_id(),
            text: entry.get_text().to_string(),
            author_name: entry.get_author_name().map(|s| s.to_string()),
            is_bot: entry.is_bot(),
        }
    }
}

impl DialogEntry for MessageData {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_author_name(&self) -> Option<&str> {
        self.author_name.as_deref()
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn is_bot(&self) -> bool {
        self.is_bot
    }
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    Standalone { message: MessageData },
    Reply { message: MessageData, reply_to: MessageData },
}

impl DialogEntry for DialogMessage {
    fn get_id(&self) -> i32 {
        match self {
            DialogMessage::Standalone { message, .. } => message.get_id(),
            DialogMessage::Reply { message, .. } => message.get_id(),
        }
    }

    fn get_author_name(&self) -> Option<&str> {
        match self {
            DialogMessage::Standalone { message, .. } => message.get_author_name(),
            DialogMessage::Reply { message, .. } => message.get_author_name(),
        }
    }

    fn get_text(&self) -> &str {
        match self {
            DialogMessage::Standalone { message, .. } => message.get_text(),
            DialogMessage::Reply { message, .. } => message.get_text(),
        }
    }

    fn is_bot(&self) -> bool {
        match self {
            DialogMessage::Standalone { message, .. } => message.is_bot(),
            DialogMessage::Reply { message, .. } => message.is_bot(),
        }
    }
}
