use anyhow::Result;

use crate::repositories::forbidden_word_repo::ForbiddenWordRepository;

#[derive(Clone)]
pub struct ForbiddenWordService {
    repo: ForbiddenWordRepository,
}

impl ForbiddenWordService {
    pub fn new(repo: ForbiddenWordRepository) -> Self {
        Self { repo }
    }

    pub async fn add_word(&self, word: &str) -> Result<()> {
        self.repo.add_word(word).await
    }

    pub async fn delete_word(&self, id: i32) -> Result<()> {
        self.repo.delete_word(id).await
    }

    pub async fn list_words(&self) -> Result<Vec<(i32, String)>> {
        self.repo.list_words().await
    }

    pub async fn contains_forbidden_word(&self, text: &str) -> Result<bool> {
        self.repo.contains_forbidden_word(text).await
    }
} 