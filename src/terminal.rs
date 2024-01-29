pub struct Terminal {
    output: String,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn write(&mut self, c: char) {
        self.output.push(c);
    }

    pub fn flush_out(&mut self) -> String {
        let out = self.output.clone();
        self.output.clear();
        out
    }
}
