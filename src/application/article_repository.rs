use crate::domain::article::Article;

pub trait ArticleRepository {
    fn get(&self, id: i32) -> Article;
}
