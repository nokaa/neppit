use thread::Thread;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    short_name: String,
    long_name: String,
    description: String,
    // The currently active threads for this board
    active_threads: Option<Vec<Thread>>,
}

impl Board {
    pub fn get_thread(&self, thread: &String) -> Option<&Thread> {
        if let Some(ref threads) = self.active_threads {
            let thread = usize::from_str_radix(thread, 10).unwrap();
            for t in threads {
                if t.thread_number == thread {
                    return Some(t);
                }
            }
        }

        None
    }
}
