pub struct Config {
    pub verbose: bool,
    pub interactive: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            interactive: false,
        }
    }
}
