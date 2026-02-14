use std::time::Instant;

pub struct Profile {
    name: String,
    start_time: Option<Instant>,
}

impl Profile {
    pub fn start(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            start_time: Some(Instant::now()),
        };
    }

    pub fn report(self, message: &str) {
        let elapsed = self.start_time.unwrap().elapsed();
        println!("[{}] {} in {:.3?}", self.name, message, elapsed);
    }

    pub fn end(self) {
        self.report("solved");
    }
}