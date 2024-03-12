use crossterm::{
    event::{read, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use super::input_processor;

pub struct Terminal {
    pub input_processor: input_processor::InputProcessor,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            input_processor: input_processor::InputProcessor::new(),
        }
    }

    fn run_before(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        self.input_processor.display().clear_screen(">")?;
        Ok(())
    }

    fn run_after(&mut self) -> std::io::Result<()> {
        self.input_processor.display().clear_screen("Goodbye!")?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.run_before()?;

        loop {
            loop {
                if let Event::Key(key_event) = read()? {
                    match self.input_processor.processor(key_event) {
                        Ok(code) => {
                            if code == "break" {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                self.input_processor.display().flush_screen()?;
            }
            if let Some(s) = self.input_processor.receiver().get_to_string() {
                if s == "exit" {
                    break;
                }
            }
        }

        self.run_after()?;
        Ok(())
    }
}
