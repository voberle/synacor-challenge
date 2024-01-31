use std::io;

// A way to access the terminal from the code, which can also be used in tests.
pub struct Terminal {
    output: String,
    // Print each character to real terminal as they come
    print: bool,

    input: String,
}

impl Terminal {
    // In real app, set print to true to have output characters go to the real terminal.
    pub fn new(print: bool) -> Self {
        Self {
            output: String::new(),
            print,
            input: String::new(),
        }
    }

    // Write a char to terminal.
    pub fn write(&mut self, c: char) {
        if self.print {
            print!("{}", c);
        }
        self.output.push(c);
    }

    // Read a char from terminal
    pub fn read(&mut self) -> char {
        if self.input.is_empty() {
            io::stdin()
                .read_line(&mut self.input)
                .expect("Failed to read input");
        }
        self.input.remove(0)
    }

    // Get all that went to terminal, and clears it.
    #[cfg(test)]
    pub fn flush_out(&mut self) -> String {
        let out = self.output.clone();
        self.output.clear();
        out
    }

    // Set what should be read from terminal.
    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
    }

    pub fn is_input_empty(&self) -> bool {
        self.input.is_empty()
    }
}
