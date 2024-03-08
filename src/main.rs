use std::io::stdout;

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};

fn main() -> std::io::Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("hello world."))?
        .execute(ResetColor)?;

    Ok(())
}
