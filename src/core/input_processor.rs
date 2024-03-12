use std::{io::Result, process::Command};

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
        self.input_receiver.insert(); // 保存输入的字符串
        self.run_command()?;
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

    /// 执行命令
    pub fn run_command(&mut self) -> Result<()> {
        if let Some(command_string) = self.input_receiver.get_to_string() {
            let output = Command::new("sh").arg("-c").arg(command_string).output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        let result = &String::from_utf8_lossy(&output.stdout);
                        self.content_display.draw_command_result(&result)?;
                    } else {
                        self.content_display
                            .draw_command_result(&String::from_utf8_lossy(&output.stderr))?;
                    }
                }
                Err(e) => {
                    self.content_display
                        .draw_command_result(&format!("command error: {}", e))?;
                }
            }
        }

        Ok(())
    }
}
