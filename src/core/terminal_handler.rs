use std::io::{self, Stdout, Write};

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use super::{content_display::ContentDisplay, input_receiver};

pub struct Terminal {
    pub out: Stdout,
    pub input_receiver: input_receiver::Input,
    pub content_display: ContentDisplay,
}

impl Terminal {
    pub fn new() -> Self {
        let out = io::stdout();
        let input_receiver = input_receiver::Input::new();
        let content_display = ContentDisplay::new();

        Self {
            out,
            input_receiver,
            content_display,
        }
    }

    fn run_before(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        self.out.execute(EnterAlternateScreen)?;
        self.content_display.draw_input_command(&mut self.out, "")?;
        Ok(())
    }

    fn run_after(&mut self) -> std::io::Result<()> {
        self.out.execute(LeaveAlternateScreen)?;
        self.out
            .execute(Clear(crossterm::terminal::ClearType::All))?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.run_before()?;

        loop {
            loop {
                if let Event::Key(key_event) = read()? {
                    match key_event.code {
                        KeyCode::Enter => {
                            self.input_receiver.insert();
                            self.content_display.draw_enter(
                                &mut self.out,
                                (self.input_receiver.get_index()) as usize,
                                &self
                                    .input_receiver
                                    .get_to_string()
                                    .expect("get input command error"),
                            )?;
                            break;
                        }
                        KeyCode::Char(c) => {
                            self.input_receiver.push_char(c);
                            self.content_display.draw_input_command(
                                &mut self.out,
                                &self.input_receiver.get_chars_to_string(),
                            )?;
                        }
                        KeyCode::Backspace => {
                            self.input_receiver.pop_char();
                            self.content_display.draw_input_command(
                                &mut self.out,
                                &self.input_receiver.get_chars_to_string(),
                            )?;
                        }
                        _ => {}
                    }
                }
                self.out.flush()?;
            }
            if let Some(s) = self.input_receiver.get_to_string() {
                if s == "exit" {
                    break;
                }
            }
        }

        self.run_after()?;
        Ok(())
    }
}
