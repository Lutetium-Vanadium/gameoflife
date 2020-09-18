use crossterm::{cursor, queue, style, terminal, ExecutableCommand};
use std::io::Write;

pub fn show(stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
    let (width, height) = terminal::size()?;

    queue!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, (height - 14) / 2)
    )?;

    print("HELP MENU", stdout, width)?;
    print("----------------------\n", stdout, width)?;
    print("h to show this menu", stdout, width)?;
    print("esc/q to quit", stdout, width)?;
    print("r to reset", stdout, width)?;
    print("e to edit board", stdout, width)?;
    queue!(stdout, cursor::MoveToNextLine(2));
    print("Editing", stdout, width)?;
    print("------------", stdout, width)?;
    print("A single character represents 2 blocks, so:", stdout, width)?;
    print("Right click to place upper block", stdout, width)?;
    print("Left click to place lower block", stdout, width)?;

    stdout.execute(cursor::RestorePosition)?;
    Ok(())
}

fn print(s: &str, stdout: &mut std::io::Stdout, width: u16) -> crossterm::Result<()> {
    queue!(
        stdout,
        cursor::MoveRight((width - s.len() as u16) / 2),
        style::Print(s),
        cursor::MoveToNextLine(1),
    )
}
