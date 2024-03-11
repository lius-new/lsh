use lsh::terminal_handler;

fn main() -> std::io::Result<()> {
    let mut terminal = terminal_handler::Terminal::new();
    terminal.run()?;
    Ok(())
}
