use std::io;

pub struct Terminal {
    output: String,
    // Print each character to real terminal as they come
    print: bool,

    input: String,
}

impl Terminal {
    pub fn new(print: bool) -> Self {
        Self {
            output: String::new(),
            print,
            input: String::new(),
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

    pub fn read(&mut self) -> char {
        if self.input.is_empty() {
            io::stdin()
                .read_line(&mut self.input)
                .ok()
                .expect("Failed to read input");
        }
        self.input.remove(0)
    }
}
