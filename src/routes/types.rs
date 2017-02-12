#[derive(FromForm)]
pub struct NewThread {
    pub subject: String,
    pub content: String,
}

#[derive(FromForm)]
pub struct NewPost {
    pub content: String,
}
