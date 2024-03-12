use std::io::Result;

use super::{content_display::ContentDisplay, input_receiver};

use crossterm::event::{KeyCode, KeyEvent};

pub struct InputProcessor {
    input_receiver: input_receiver::Input,
    content_display: ContentDisplay,
}

impl InputProcessor {
    /// new: 构建InputProcessor
    pub(super) fn new() -> Self {
        let input_receiver = input_receiver::Input::new();
        let content_display = ContentDisplay::new();

        Self {
            input_receiver,
            content_display,
        }
    }
    // receiver: 获取input_receiver
    pub fn receiver(&mut self) -> &mut input_receiver::Input {
        &mut self.input_receiver
    }

    // display: 获取display
    pub fn display(&mut self) -> &mut ContentDisplay {
        &mut self.content_display
    }

    pub fn processor(&mut self, key_event: KeyEvent) -> Result<&str> {
        match key_event.code {
            KeyCode::Enter => self.enter_handler(),
            KeyCode::Char(c) => self.input_char_handler(c),
            KeyCode::Backspace => self.backspace_handler(),
            _ => Ok("proceed"),
        }
    }

    /// enter_handler: 处理回车事件
    pub fn enter_handler(&mut self) -> Result<&str> {
        self.input_receiver.insert();
        self.content_display.draw_enter(
            (self.input_receiver.get_index()) as usize,
            &self
                .input_receiver
                .get_to_string()
                .expect("get input command error"),
        )?;
        Ok("break")
    }

    /// input_char_handler: 处理输入字符事件
    pub fn input_char_handler(&mut self, c: char) -> Result<&str> {
        self.input_receiver.push_char(c);
        self.content_display
            .draw_input_command(&self.input_receiver.get_chars_to_string())?;
        Ok("proceeed")
    }

    /// backspace_handler: 处理删除事件
    pub fn backspace_handler(&mut self) -> Result<&str> {
        self.input_receiver.pop_char();
        self.content_display
            .draw_input_command(&self.input_receiver.get_chars_to_string())?;
        Ok("proceeed")
    }
}
