use crossterm::{cursor, event, execute, queue, style, terminal};
use std::io::Write;
mod board;
mod help;

fn run() -> crossterm::Result<()> {
    let mut stdout = std::io::stdout();
    let mut state = State::Board;
    let dur = std::time::Duration::from_millis(100);

    let (width, height) = terminal::size()?;
    let mut board = board::Board::new(width as usize, (2 * height) as usize - 2);

    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
    )?;

    loop {
        if event::poll(dur)? {
            match event::read()? {
                event::Event::Key(e) => match e.code {
                    event::KeyCode::Char(c) => {
                        if e.modifiers.contains(event::KeyModifiers::CONTROL) && c == 'c' {
                            return Ok(());
                        }

                        match c {
                            'h' => {
                                queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
                                state = State::Help;
                            }
                            'b' => state = State::Board,
                            'q' => return Ok(()),
                            'r' => board.reset(),
                            'e' => return Ok(()), // unimplemented
                            _ => {}
                        }
                    }
                    event::KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }

        match state {
            State::Help => {
                help::show(&mut stdout)?;
                queue!(
                    stdout,
                    cursor::SavePosition,
                    cursor::MoveTo(1, height - 1),
                    style::Print("type b to go back"),
                    cursor::RestorePosition,
                )?;
            }
            State::Board => {
                board.next();
                board.display(&mut stdout)?;
                let text = format!("renders: {}", board.renders);
                queue!(
                    stdout,
                    cursor::SavePosition,
                    cursor::MoveTo(1, height - 1),
                    style::Print("type h for help"),
                    cursor::MoveTo(width - (text.len() as u16) - 1, height - 1),
                    style::Print(text),
                    cursor::RestorePosition,
                )?;
            }
        }

        stdout.flush()?;
    }
}

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    run()?;
    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), cursor::Show)?;

    Ok(())
}

#[derive(PartialEq)]
enum State {
    Board,
    Help,
}
