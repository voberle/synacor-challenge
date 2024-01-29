pub struct Terminal {
    output: String,
    // Print each character to real terminal as they come
    print: bool,
}

impl Terminal {
    pub fn new(print: bool) -> Self {
        Self {
            output: String::new(),
            print,
        }
    }

    pub fn write(&mut self, c: char) {
        if self.print {
            print!("{}", c);
        }
        self.output.push(c);
    }

    pub fn flush_out(&mut self) -> String {
        let out = self.output.clone();
        self.output.clear();
        out
    }
}
