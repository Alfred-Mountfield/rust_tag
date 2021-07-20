#[derive(Debug, Clone)]
pub struct Agent {
    pub tagged: bool,
}

impl Default for Agent {
    fn default() -> Self {
        Agent {
            tagged: false
        }
    }
}
