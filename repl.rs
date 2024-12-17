use std::io::{self, Write, BufRead};

/*
To make the REPL, fundamentally, - what we're going to need is:
    - a while(True) loop that continues until it sees a ';'
    - ability to enter multi-line commands
    - graphically interface in a nice usable way (i.e. prompting all user inputs with '>>')
    - Ability to return string of full command to be executed after tokenization
 */

pub struct Repl {
    buffer: String,
    history: Vec<String>,
    is_in_multiline: bool,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            buffer: String::new(),
            history: Vec::<String>::new(),
            is_in_multiline: false,
        }
    }

    fn clean_input(input: &mut String) {
        // Remove \n, \r
        *input = input.trim_end_matches(&['\n', '\r'][..]).to_string();

        // If there is a Some ';' in the string to be cleaned, find it, include it and get rid of everything after
        if let Some(pos) = input.find(';') {
            *input = input[..pos + 1].to_string();
        }
    }

    // Made this function before realizing that .join just exists
    fn multiline_to_singleline(history: &mut Vec<String>) -> String {
        history.join(" ")
    }

    pub fn main_loop(&mut self) -> String {
        let stdin = io::stdin();

        // Lock the handle for buffered reading throughout
        let mut handle = stdin.lock();

        self.is_in_multiline = false;

        // Main loop for handling all inputs until a conclusion is reached
        loop {
            // Clear line so the buffer doesn't extend ad infinitum
            self.buffer.clear();

            if self.is_in_multiline {
                print!("..");
            } else {
                // Display the prompt
                print!(">>");
            }

            // Flush stdout to ensure the prompt appears immediately
            io::stdout().flush().expect("Failed to flush stdout");

            // Read line into the buffer
            let bytes_read = handle
                .read_line(&mut self.buffer)
                .expect(":( Womp Womp");

            // In case Ctrl+C or Ctrl+Z
            if bytes_read == 0 {
                println!("\nExiting REPL");
                break;
            }

            // End the command when it sees a ;
            // Also need to cut off anything past the ';'
            if self.buffer.contains(';') {
                Self::clean_input(&mut self.buffer);
                self.history.push(self.buffer.clone());
                self.is_in_multiline = false;
                break;
            }
            Self::clean_input(&mut self.buffer);
            self.history.push(self.buffer.clone());
            self.is_in_multiline = true;
        }
        // Where we can actually access and return the input
        let result: String = Self::multiline_to_singleline(&mut self.history);
        print!("Input is {:?}", result);
        return result
    }
}
