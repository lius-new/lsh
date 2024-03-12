use core::time;
use std::io::{stdout, Result, Stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, terminal::Clear, ExecutableCommand};

pub struct Cursor {
    x: usize,
    y: usize,
}
impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_cursor_x(&mut self, x: usize) {
        self.x = x
    }
    pub fn move_cursor_y(&mut self, y: usize) {
        self.y = y
    }
    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.move_cursor_x(x);
        self.move_cursor_y(y);
    }
}

pub struct ContentDisplay {
    out: Stdout,
    pub cursor: Cursor,
}

impl ContentDisplay {
    pub fn new() -> Self {
        Self {
            out: stdout(),
            cursor: Cursor::new(),
        }
    }

    /// clear_screen: 清空屏幕
    /// 提供一个字符串，在清空屏幕后唯一显示的就是该字符串
    pub fn clear_screen(&mut self, content: &str) -> Result<()> {
        self.out
            .execute(Clear(crossterm::terminal::ClearType::All))?;
        self.cursor.move_cursor(0, 0);
        self.draw_cursor()?;
        self.out.execute(Print(content))?;
        Ok(())
    }

    /// draw_input_command: 绘制输入的命令
    pub fn draw_input_command(&mut self, content: &str) -> Result<()> {
        self.out
            .execute(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        self.cursor.move_cursor_x(0);
        self.draw_cursor()?;
        self.out.execute(Print(">"))?;
        self.out.execute(Print(content))?;
        Ok(())
    }

    /// draw_command_result: 绘制输出的结果
    pub fn draw_command_result(&mut self, content: &str) -> Result<()> {
        self.cursor.move_cursor_y(self.cursor.y + 1);
        self.draw_cursor()?;

        let trim_end = content.trim_end_matches("\n");
        for ele in String::from(trim_end).split("\n") {
            self.cursor.move_cursor_y(self.cursor.y + 1);
            self.out.execute(Print(ele))?;
            self.draw_cursor()?;
        }

        self.cursor.move_cursor_y(self.cursor.y + 1);
        self.draw_cursor()?;
        self.out.execute(Print(">"))?;
        Ok(())
    }

    /// draw_enter: 绘制终端
    pub fn draw_cursor(&mut self) -> Result<()> {
        self.out
            .execute(MoveTo(self.cursor.x as u16, self.cursor.y as u16))?;
        Ok(())
    }

    /// flush_screen: 刷新屏幕
    pub fn flush_screen(&mut self) -> Result<()> {
        self.out.flush()?;
        Ok(())
    }
}
