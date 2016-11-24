#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_number: usize,
    pub content: String,
    pub name: String,
    pub email: String,
}
