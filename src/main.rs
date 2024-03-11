use lsh::terminal_handler;

fn main() -> std::io::Result<()> {
    // let mut out = stdout();
    // enable_raw_mode()?;

    // out.execute(EnterAlternateScreen)?; // 进入备用屏幕
    // out.execute(Hide)?;
    // out.execute(MoveTo(0, 0))?;

    // loop {
    //     out.execute(MoveToColumn(0))?;
    //     out.execute(Print(">"))?;
    //     out.flush()?;

    //     let mut input = String::new();
    //     loop {
    //         if let Event::Key(key_event) = read()? {
    //             match key_event.code {
    //                 KeyCode::Enter => {
    //                     break;
    //                 }
    //                 KeyCode::Char(c) => {
    //                     input.push(c);
    //                     out.execute(Clear(ClearType::CurrentLine))?;
    //                     out.execute(MoveToColumn(0))?;
    //                     out.execute(Print(">"))?;
    //                     out.execute(Print(&input))?;
    //                     out.flush()?;
    //                 }
    //                 KeyCode::Backspace => {
    //                     input.pop().expect("msg");
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     if input == "exit" {
    //         break;
    //     }
    //     out.flush()?;
    // }

    // out.execute(Show)?;
    // out.execute(LeaveAlternateScreen)?; // 离开备用屏幕
    // disable_raw_mode()?; // 关闭原始模式

    // Ok(())

    let mut terminal = terminal_handler::Terminal::new();
    terminal.run()?;
    Ok(())
}
