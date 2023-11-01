use poem_openapi::Object;

pub type ArticleId = i64;

#[derive(Object, Debug)]
pub struct Article {
    #[oai(read_only)]
    pub id: ArticleId,
    #[oai(validator(min_length = 1, max_length = 250))]
    pub title: String,
    #[oai(validator(min_length = 1))]
    pub content: String,
}
