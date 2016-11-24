use post::Post;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thread {
    pub thread_number: usize,
    subject: String,
    name: String,
    email: String,
    content: String,
    childrn: Vec<Post>,
}
