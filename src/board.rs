#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    short_name: String,
    long_name: String,
    description: String,
    // The currently active threads for this board
    active_threads: Option<Vec<usize>>,
}
