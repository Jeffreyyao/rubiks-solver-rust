use std::time::*;

#[derive(Debug)]
pub struct Profile {
    name: String,
    start_time: Option<Instant>,
    print_enabled: bool,
}

impl Profile {
    pub fn start(name: &str, print_enabled: bool) -> Self {
        return Self {
            name: name.to_string(),
            start_time: Some(Instant::now()),
            print_enabled,
        };
    }

    pub fn report(&self, message: &str) {
        let elapsed = self.start_time.unwrap().elapsed();
        if self.print_enabled {
            println!("[{}] {} in {:.3?}", self.name, message, elapsed);
        }
    }

    pub fn end(self) -> Duration {
        if self.print_enabled {
            self.report("solved");
        }
        if self.start_time.is_some() {
            self.start_time.unwrap().elapsed()
        } else {
            Duration::from_secs(0)
        }
    }
}