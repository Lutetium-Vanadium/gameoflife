use crossterm::{cursor, queue, style, ExecutableCommand};
use std::io::Write;

pub fn show(stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
    queue!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, 0),
        style::Print("      HELP MENU      "),
        cursor::MoveToNextLine(1),
        style::Print("---------------------\n"),
        cursor::MoveToNextLine(1),
        style::Print(" h to show this menu"),
        cursor::MoveToNextLine(1),
        style::Print(" esc/q to quit"),
        cursor::MoveToNextLine(1),
        style::Print(" r to reset"),
        cursor::MoveToNextLine(1),
        style::Print(" e to edit board"),
    )?;

    stdout.execute(cursor::RestorePosition)?;
    Ok(())
}
