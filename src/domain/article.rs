use poem_openapi::Object;

#[derive(Object, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
}
