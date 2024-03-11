use std::{
    collections::HashMap,
    io::{self, Stdout, Write},
};

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use super::content_display::ContentDisplay;

pub struct Terminal {
    pub out: Stdout,
    pub history: HashMap<u32, String>,
    pub content_display: ContentDisplay,
}

impl Terminal {
    pub fn new() -> Self {
        let out = io::stdout();
        let history = HashMap::new();
        let content_display = ContentDisplay::new();

        Self {
            out,
            history,
            content_display,
        }
    }

    fn run_before(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        self.out.execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn run_after(&mut self) -> std::io::Result<()> {
        self.out.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn run(&mut self) -> std::io::Result<()> {
        self.run_before()?;

        // run process
        let mut history_id = 0;
        loop {
            history_id += 1;
            let mut chars = Vec::new();

            loop {
                if let Event::Key(key_event) = read()? {
                    match key_event.code {
                        KeyCode::Enter => {
                            self.history.insert(history_id, String::from_iter(&chars));
                            break;
                        }
                        KeyCode::Char(c) => {
                            chars.push(c);
                            self.content_display
                                .draw_input_command(&mut self.out, String::from_iter(&chars))?;
                        }
                        KeyCode::Backspace => {
                            chars.pop().expect("msg");
                        }
                        _ => {}
                    }
                }
                self.out.flush()?;
            }
            if String::from_iter(&chars).eq("exit") {
                break;
            }
        }

        self.run_after()?;
        Ok(())
    }
}
