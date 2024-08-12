use PasteError::ContentTooLarge;

#[derive(Debug, Clone)]
pub struct Paste {
    slug: Option<String>,
    content: String,
}

pub enum PasteError {
    ContentTooLarge
}

impl Paste {
    pub fn new(content: String) -> Result<Self, PasteError> {
        // Invariants can be enforced here
        if content.len() > 1024 * 1024 {
            return Err(ContentTooLarge)
        }

        Ok(Self {
            slug: None,
            content,
        })
    }

    pub fn get_slug(&self) -> Option<&str> {
        (&self.slug).as_deref()
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}