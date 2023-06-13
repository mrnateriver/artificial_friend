pub trait DialogEntry {
    fn get_id(&self) -> i32;
    fn get_text(&self) -> &str;
    fn get_author_name(&self) -> Option<&str>;
}

#[derive(Debug, Clone)]
pub struct MessageData {
    id: i32,
    text: String,
    author_name: Option<String>,
}

impl MessageData {
    pub fn new(id: i32, text: String, author_name: Option<String>) -> Self {
        Self { id, text, author_name }
    }

    pub fn from_entry(entry: &impl DialogEntry) -> Self {
        Self {
            id: entry.get_id(),
            text: entry.get_text().to_string(),
            author_name: entry.get_author_name().map(|s| s.to_string()),
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
}
