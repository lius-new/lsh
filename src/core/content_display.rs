use std::io::{Result, Stdout};

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
    pub cursor: Cursor,
}

impl ContentDisplay {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(),
        }
    }

    /// draw_enter: 绘制回车的命令
    pub fn draw_enter(&mut self, out: &mut Stdout, y: usize, content: &str) -> Result<()> {
        self.cursor.move_cursor_y(y - 1);
        self.draw_cursor(out)?;
        out.execute(Print(content))?;
        self.cursor.move_cursor_y(y);
        self.draw_cursor(out)?;
        out.execute(Print(">"))?;
        Ok(())
    }

    /// draw_input_command: 绘制输入的命令
    pub fn draw_input_command(&mut self, out: &mut Stdout, content: &str) -> Result<()> {
        out.execute(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        self.cursor.move_cursor_x(0);
        self.draw_cursor(out)?;
        out.execute(Print(">"))?;
        out.execute(Print(content))?;
        Ok(())
    }

    /// draw_enter: 绘制终端
    pub fn draw_cursor(&self, out: &mut Stdout) -> Result<()> {
        out.execute(MoveTo(self.cursor.x as u16, self.cursor.y as u16))?;
        Ok(())
    }
}
