use std::io;

// A way to access the terminal from the code, which can also be used in tests.
pub struct Terminal {
    output: String,
    // Print each character to real terminal as they come
    print: bool,

    input: String,
    interactive_mode: bool,
}

impl Terminal {
    // In real app, set print to true to have output characters go to the real terminal.
    pub fn new(print: bool) -> Self {
        Self {
            output: String::new(),
            print,
            input: String::new(),
            interactive_mode: false,
        }
    }

    // Write a char to terminal.
    pub fn write(&mut self, c: char) {
        if self.print {
            print!("{}", c);
        }
        self.output.push(c);
    }

    // Read a char from terminal.
    // The terminal input is cached in `self.input`: If that is not empty, return the first char from it.
    // If it's empty, read from stdin and fill the cache with the read line.
    // If the read line starts with '>', just return None.
    pub fn read(&mut self) -> Option<char> {
        if self.input.is_empty() {
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read input");
            if buf.starts_with('>') {
                self.interactive_mode = true;
                return None;
            } else {
                self.input = buf;
            }
        }
        Some(self.input.remove(0))
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

    pub fn is_interactive_mode(&self) -> bool {
        self.interactive_mode
    }

    pub fn quit_interactive_mode(&mut self) {
        self.interactive_mode = false;
    }
}
