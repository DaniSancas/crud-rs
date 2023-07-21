use crate::domain::article::Article;
use eyre::Result;

pub trait ArticleRepository {
    fn get(&self, id: i32) -> Result<Article>;
}
