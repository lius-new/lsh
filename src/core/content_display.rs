use std::io::{Result, Stdout};

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo},
    style::Print,
    terminal::Clear,
    ExecutableCommand,
};

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
    cursor: Cursor,
}

impl ContentDisplay {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(),
        }
    }

    pub fn draw_input_command(&mut self, out: &mut Stdout, content: String) -> Result<()> {
        out.execute(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        self.cursor.move_cursor_x(0);
        self.draw_cursor(out)?;
        out.execute(Print(content))?;
        Ok(())
    }
    pub fn draw_cursor(&self, out: &mut Stdout) -> Result<()> {
        out.execute(MoveTo(self.cursor.x as u16, self.cursor.y as u16))?;
        Ok(())
    }
}
