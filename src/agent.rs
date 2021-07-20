#[derive(Debug, Clone)]
pub struct Agent {
    tagged: bool
}

impl Default for Agent {
    fn default() -> Self {
        Agent {
            tagged: false
        }
    }
}