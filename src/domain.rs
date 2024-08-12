use crate::domain::PasteError::SlugTooLarge;
use PasteError::ContentTooLarge;

#[derive(Debug, Clone)]
pub struct Paste {
    slug: String,
    content: String,
}

pub enum PasteError {
    ContentTooLarge,
    SlugTooLarge,
}

impl Paste {
    pub fn new(slug: String, content: String) -> Result<Self, PasteError> {
        // Invariants can be enforced here
        if slug.len() > 256 {
            return Err(SlugTooLarge);
        }
        if content.len() > 1024 * 1024 {
            return Err(ContentTooLarge);
        }

        Ok(Self { slug, content })
    }

    pub fn get_slug(&self) -> &str {
        &self.slug
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}
